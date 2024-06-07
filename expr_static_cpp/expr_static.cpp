#include <cstdlib>
#include <concepts>
#include <iostream>
#include <memory>
#include <optional>
#include <stdexcept>
#include <string>
#include <utility>
#include <variant>

namespace {

enum class op1_kind {
    minus,
};

enum class op2_kind {
    add,
    sub,
    mul,
    div,
};

template <class T> class expr {
public:
    expr(const T& v): e(v) {}
    expr(op1_kind kind, std::unique_ptr<expr<T>>&& child):
        e(std::in_place_type_t<op1>(), kind, std::move(child)) {}
    expr(op2_kind kind, std::unique_ptr<expr<T>>&& l, std::unique_ptr<expr<T>>&& r):
        e(std::in_place_type_t<op2>(), kind, std::move(l), std::move(r)) {}
    std::optional<T> eval() const {
        return std::visit([this](auto&& v) { return eval(v); }, e);
    }
    void display() const {
        return std::visit([this](auto&& v) { display(v); }, e);
    }
private:
    struct op1 {
        op1_kind op;
        std::unique_ptr<expr<T>> child;
    };
    struct op2 {
        op2_kind op;
        std::unique_ptr<expr<T>> l;
        std::unique_ptr<expr<T>> r;
    };
    std::optional<T> eval(const T& v) const {
        return v;
    }
    void display(const T& v) const {
        std::cout << '(' << v << ')';
    }
    std::optional<T> eval(const op1& v) const requires (!std::same_as<T, std::string>) {
        if (!v.child)
            throw std::invalid_argument("child is nullptr");
        auto e = v.child->eval();
        if (!e)
            return std::nullopt;
        switch (v.op) {
        case op1_kind::minus:
            return -e.value();
        default:
            return std::nullopt;
        }
    }
    std::optional<T> eval(const op1&) const requires std::same_as<T, std::string> {
        return std::nullopt;
    }
    void display(const op1& v) const {
        if (!v.child)
            throw std::invalid_argument("child is nullptr");
        std::cout << '(';
        switch (v.op) {
        case op1_kind::minus:
            std::cout << '-';
            break;
        default:
            break;
        }
        v.child->display();
        std::cout << ')';
    }
    std::optional<T> eval(const op2& v) const {
        if (!v.l)
            throw std::invalid_argument("left is nullptr");
        if (!v.r)
            throw std::invalid_argument("right is nullptr");
        auto el = v.l->eval();
        auto er = v.r->eval();
        if (el && er)
            return eval(v.op, el.value(), er.value());
        else
            return std::nullopt;
    }
    std::optional<T> eval(op2_kind op, const T& el, const T& er) const requires (!std::same_as<T, std::string>) {
        switch (op) {
        case op2_kind::add:
            return el + er;
        case op2_kind::sub:
            return el - er;
        case op2_kind::mul:
            return el * er;
        case op2_kind::div:
            if (er == 0)
                return std::nullopt;
            else
                return el / er;
        default:
            return std::nullopt;
        }
    }
    std::optional<T> eval(op2_kind op, const T& el, const T& er) const requires std::same_as<T, std::string> {
        switch (op) {
        case op2_kind::add:
            return el + er;
        case op2_kind::sub:
        case op2_kind::mul:
        case op2_kind::div:
        default:
            return std::nullopt;
        }
    }
    void display(const op2& v) const {
        if (!v.l)
            throw std::invalid_argument("left is nullptr");
        if (!v.r)
            throw std::invalid_argument("right is nullptr");
        std::cout << '(';
        v.l->display();
        switch (v.op) {
        case op2_kind::add:
            std::cout << '+';
            break;
        case op2_kind::sub:
            std::cout << '-';
            break;
        case op2_kind::mul:
            std::cout << '*';
            break;
        case op2_kind::div:
            std::cout << '/';
            break;
        default:
            break;
        }
        v.r->display();
        std::cout << ')';
    }
    std::variant<T, op1, op2> e;
};

namespace parser {

template <class T> std::unique_ptr<expr<T>> expression();

template <class T> std::unique_ptr<expr<T>> terminal()
{
    T v;
    if (std::cin >> v)
        return std::make_unique<expr<T>>(v);
    return nullptr;
}

template <class T> std::unique_ptr<expr<T>> factor()
{
    std::optional<op1_kind> op{};
    if (char c; std::cin >> c) {
        switch (c) {
        case '-':
            op = op1_kind::minus;
            break;
        case '(':
            std::cin.unget();
            break;
        default:
            std::cin.unget();
            break;
        }
        if (char c; std::cin >> c) {
            std::unique_ptr<expr<T>> e;
            switch (c) {
            case '(':
                e = expression<T>();
                if (e) {
                    if (char c; std::cin >> c)
                        switch (c) {
                        case ')':
                            break;
                        default:
                            return nullptr;
                        }
                    else
                        return nullptr;
                } else
                    return nullptr;
                break;
            default:
                std::cin.unget();
                e = terminal<T>();
                break;
            }
            if (op)
                e = std::make_unique<expr<T>>(op.value(), std::move(e));
            return e;
        }
    }
    return nullptr;
}

template <class T> std::unique_ptr<expr<T>> term()
{
    if (auto f1 = factor<T>()) {
        for (char c; std::cin >> c;) {
            std::optional<op2_kind> op{};
            switch (c) {
            case '*':
                op = op2_kind::mul;
                break;
            case '/':
                op = op2_kind::div;
                break;
            default:
                std::cin.unget();
                return f1;
            }
            if (auto f2 = factor<T>())
                f1 = std::make_unique<expr<T>>(op.value(), std::move(f1), std::move(f2));
            else
                return nullptr;
        }
        return f1;
    } else
        return nullptr;
}

template <class T> std::unique_ptr<expr<T>> expression()
{
    if (auto t1 = term<T>()) {
        for (char c; std::cin >> c;) {
            std::optional<op2_kind> op{};
            switch (c) {
            case '+':
                op = op2_kind::add;
                break;
            case '-':
                op = op2_kind::sub;
                break;
            default:
                std::cin.unget();
                return t1;
            }
            if (auto t2 = term<T>())
                t1 = std::make_unique<expr<T>>(op.value(), std::move(t1), std::move(t2));
            else
                return nullptr;
        }
        return t1;
    } else
        return nullptr;
}

} // namespace parser

template <class T> std::unique_ptr<const expr<T>> parse()
{
    auto p = parser::expression<T>();
    if (char c; std::cin >> c)
        return nullptr;
    return p;
}

template <class T> int run()
{
    try {
        if (auto e = parse<T>()) {
            e->display();
            std::cout << '\n';
            if (auto v = e->eval())
                std::cout << *v << '\n';
            else
                std::cout << "no value\n";
        } else
            std::cout << "invalid expression\n";
       
    } catch (const std::exception& e) {
        std::cerr << "Unexpected exception: " << e.what() << std::endl;
        return EXIT_FAILURE;
    }
    return EXIT_SUCCESS;
}

int usage(char* argv0)
{
    std::cerr << "usage: " << argv0 << R"( {i|u|d|s}

Evaluates an expression read from stdin consisting of:
- values
- unary operator -
- binary operators +, -, *, /
- parentheses
- whitespace (ignored)

Selection of type:

i = int
u = unsigned
d = double
s = std::string (only binary +, whitespace needed around operators and
    parentheses)

)";
    return EXIT_FAILURE;
}

} // namespace

int main(int argc, char* argv[])
{
    if (argc != 2)
        return usage(argv[0]);
    switch (argv[1][0]) {
    case 'i':
        return run<int>();
    case 'u':
        return run<unsigned>();
    case 'd':
        return run<double>();
    case 's':
        return run<std::string>();
    default:
        return usage(argv[0]);
    }
}
