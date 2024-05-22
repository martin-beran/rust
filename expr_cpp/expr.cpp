#include <cstdlib>
#include <iostream>
#include <memory>
#include <optional>
#include <stdexcept>
#include <string>
#include <utility>

namespace {

template <class T> class expr {
public:
    expr() = default;
    expr(const expr&) = delete;
    expr(expr&&) = delete;
    expr& operator=(const expr&) = delete;
    expr& operator=(expr&&) = delete;
    virtual ~expr() = default;
    virtual std::optional<T> eval() const = 0;
    virtual void display() const = 0;
};

template <class T> class value: public expr<T> {
public:
    value(const T& v): v{v} {}
    std::optional<T> eval() const override {
        return v;
    }
    void display() const override {
        std::cout << '(' << v << ')';
    }
private:
    T v;
};

template <class T> class op1: public expr<T> {
public:
    using expr_t = expr<T>;
    using child_ptr = std::shared_ptr<expr_t>;
    std::optional<T> eval() const override {
        if (auto v = child().eval())
            return eval_op(*v);
        else
            return std::nullopt;
    }
    void display() const override {
        std::cout << '(' << display_op();
        child().display();
        std::cout << ')';
    }
    void child(child_ptr&& p) {
        if (!p)
            throw std::invalid_argument("child is nullptr");
        c = std::move(p);
    }
protected:
    const expr_t& child() const {
        if (!c)
            throw std::invalid_argument("child is nullptr");
        return *c;
    }
    virtual std::optional<T> eval_op(const T& v) const = 0;
    virtual char display_op() const = 0;
private:
    child_ptr c{};
};

template <class T> class op_minus: public op1<T> {
protected:
    std::optional<T> eval_op(const T& v) const override {
        return -v;
    }
    char display_op() const override {
        return '-';
    }
};

template <> class op_minus<std::string>: public op1<std::string> {
protected:
    std::optional<std::string> eval_op(const std::string&) const override {
        return std::nullopt;
    }
    char display_op() const override {
        return '-';
    }
};

template <class T> class op2: public expr<T> {
public:
    using expr_t = expr<T>;
    using child_ptr = std::shared_ptr<expr_t>;
    std::optional<T> eval() const override {
        if (auto [a, b] = std::pair{left().eval(), right().eval()}; a && b)
            return  eval_op(*a, *b);
        else
            return std::nullopt;
    }
    void display() const override {
        std::cout << '(';
        left().display();
        std::cout << display_op();
        right().display();
        std::cout << ')';
    }
    void left(child_ptr&& p) {
        if (!p)
            throw std::invalid_argument("left is nullptr");
        l = std::move(p);
    }
    void right(child_ptr&& p) {
        if (!p)
            throw std::invalid_argument("right is nullptr");
        r = std::move(p);
    }
protected:
    const expr_t& left() const {
        if (!l)
            throw std::invalid_argument("left is nullptr");
        return *l;
    }
    const expr_t& right() const {
        if (!r)
            throw std::invalid_argument("right is nullptr");
        return *r;
    }
    virtual std::optional<T> eval_op(const T& a, const T& b) const = 0;
    virtual char display_op() const = 0;
private:
    child_ptr l{};
    child_ptr r{};
};

template <class T> class op_add: public op2<T> {
protected:
    std::optional<T> eval_op(const T& a, const T& b) const override {
        return a + b;
    }
    char display_op() const override {
        return '+';
    }
};

template <class T> class op_sub: public op2<T> {
protected:
    std::optional<T> eval_op(const T& a, const T& b) const override {
        return a - b;
    }
    char display_op() const override {
        return '-';
    }
};

template <> class op_sub<std::string>: public op2<std::string> {
protected:
    std::optional<std::string> eval_op(const std::string&,
                                       const std::string&) const override
    {
        return std::nullopt;
    }
    char display_op() const override {
        return '-';
    }
};

template <class T> class op_mul: public op2<T> {
protected:
    std::optional<T> eval_op(const T& a, const T& b) const override {
        return  a * b;
    }
    char display_op() const override {
        return '*';
    }
};

template <> class op_mul<std::string>: public op2<std::string> {
protected:
    std::optional<std::string> eval_op(const std::string&,
                                       const std::string&) const override
    {
        return std::nullopt;
    }
    char display_op() const override {
        return '*';
    }
};

template <class T> class op_div: public op2<T> {
protected:
    std::optional<T> eval_op(const T& a, const T& b) const override {
        if (b == 0)
            return std::nullopt;
        return  a / b;
    }
    char display_op() const override {
        return '/';
    }
};

template <> class op_div<std::string>: public op2<std::string> {
protected:
    std::optional<std::string> eval_op(const std::string&,
                                       const std::string&) const override
    {
        return std::nullopt;
    }
    char display_op() const override {
        return '/';
    }
};

namespace parser {

template <class T> std::shared_ptr<expr<T>> expression();

template <class T> std::shared_ptr<expr<T>> terminal()
{
    T v;
    if (std::cin >> v)
        return std::make_shared<value<T>>(v);
    return nullptr;
}

template <class T> std::shared_ptr<expr<T>> factor()
{
    std::shared_ptr<op1<T>> m;
    if (char c; std::cin >> c) {
        switch (c) {
        case '-':
            m = std::make_shared<op_minus<T>>();
            break;
        case '(':
            std::cin.unget();
            break;
        default:
            std::cin.unget();
            break;
        }
        if (char c; std::cin >> c) {
            std::shared_ptr<expr<T>> e;
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
            if (m) {
                m->child(std::move(e));
                e = std::move(m);
            }
            return e;
        }
    }
    return nullptr;
}

template <class T> std::shared_ptr<expr<T>> term()
{
    if (auto f1 = factor<T>()) {
        for (char c; std::cin >> c;) {
            std::shared_ptr<op2<T>> op{};
            switch (c) {
            case '*':
                op = std::make_shared<op_mul<T>>();
                break;
            case '/':
                op = std::make_shared<op_div<T>>();
                break;
            default:
                std::cin.unget();
                return f1;
            }
            if (auto f2 = factor<T>()) {
                op->left(std::move(f1));
                op->right(std::move(f2));
                f1 = std::move(op);
            } else
                return nullptr;
        }
        return f1;
    } else
        return nullptr;
}

template <class T> std::shared_ptr<expr<T>> expression()
{
    if (auto t1 = term<T>()) {
        for (char c; std::cin >> c;) {
            std::shared_ptr<op2<T>> op{};
            switch (c) {
            case '+':
                op = std::make_shared<op_add<T>>();
                break;
            case '-':
                op = std::make_shared<op_sub<T>>();
                break;
            default:
                std::cin.unget();
                return t1;
            }
            if (auto t2 = term<T>()) {
                op->left(std::move(t1));
                op->right(std::move(t2));
                t1 = std::move(op);
            } else
                return nullptr;
        }
        return t1;
    } else
        return nullptr;
}

} // namespace parser

template <class T> std::shared_ptr<const expr<T>> parse()
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
