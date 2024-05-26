// A graph of nodes connected by shared and weak pointers in Rust
//
// This is a reimplementation of C++ program graph_cpp.
// A classical example of a dynamic graph data structure, with ownership managed by shared and weak
// pointers. The graph topology is like in an application that dynamically creates sessions and
// registers handlers for events on each session. Handlers keep references (smart pointers) to
// sessions. If all handlers for a session are deleted (executed or canceled), the session is deleted
// as well.

#include <cstdlib>
#include <iostream>
#include <memory>
#include <optional>
#include <set>
#include <stdexcept>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

namespace {

// A registered session
class session {
public:
    session(std::string_view name): _name(name) {}
    ~session() {
        std::cout << "deleted session " << name() << std::endl;
    }
    const std::string& name() const {
        return _name;
    }
private:
    std::string _name;
};

// A registered handler
class handler {
public:
    handler(std::string_view name): _name(name) {}
    handler(std::string_view name, std::shared_ptr<session> shared_p): _name(name), shared_p(shared_p) {}
    handler(std::string_view name, std::weak_ptr<session> weak_p): _name(name), weak_p(weak_p) {}
    const std::string& name() const {
        return _name;
    }
    bool operator<(const handler& o) const {
        return name() < o.name();
    }
    std::shared_ptr<session> get_shared() const {
        if (shared_p)
            return shared_p;
        else
            return weak_p.lock();
    }
    bool is_shared() const {
        return bool(shared_p);
    }
private:
    std::string _name;
    std::shared_ptr<session> shared_p{};
    std::weak_ptr<session> weak_p{};
};

// A table of all registered handlers
class hnd_table_t {
public:
    void create_session(std::string_view handler_name, std::string_view session_name) {
        if (data.emplace(handler(handler_name, std::make_shared<session>(session_name))).second)
            std::cout << "Created" << std::endl;
        else
            std::cout << "Handler already exists" << std::endl;
    }
    void add_shared_ptr(std::string_view from, std::string_view to) {
        if (auto to_hnd = data.find(handler(to)); to_hnd == data.end())
            std::cout << "Target handler does not exist" << std::endl;
        else
            if (data.emplace(handler(from, to_hnd->get_shared())).second)
                std::cout << "Created" << std::endl;
            else
                std::cout << "Handler already exists" << std::endl;
    }
    void add_weak_ptr(std::string_view from, std::string_view to) {
        if (auto to_hnd = data.find(handler(to)); to_hnd == data.end())
            std::cout << "Target handler does not exist" << std::endl;
        else
            if (data.emplace(handler(from, std::weak_ptr<session>(to_hnd->get_shared()))).second)
                std::cout << "Created" << std::endl;
            else
                std::cout << "Handler already exists" << std::endl;
    }
    void erase(std::string_view handler_name) {
        if (data.erase(handler(handler_name)))
            std::cout << "Handler erased" << std::endl;
        else
            std::cout << "Handler does not exist" << std::endl;
    }
    void display() {
        for (auto&& h: data) {
            std::cout << h.name();
            if (auto ptr = h.get_shared())
                std::cout << (h.is_shared() ? " => " : " -> ") << ptr->name();
            std::cout << std::endl;
        }
    }
private:
    std::set<handler> data;
};

auto split(std::string_view s)
{
    constexpr auto npos = std::string_view::npos;
    std::string_view ws{" \t"};
    std::vector<std::string_view> res;
    for (;;) {
        if (auto b = s.find_first_not_of(ws); b == npos)
            break;
        else
            s = s.substr(b);
        if (s.empty())
            break;
        if (auto e = s.find_first_of(ws); e == npos) {
            res.push_back(s);
            break;
        } else {
            res.push_back(s.substr(0, e));
            s = s.substr(e);
        }
    }
    return res;
}

// The main loop.
//
// It reads and process lines with commands from stdin and executes them.
int run()
{
    hnd_table_t hnd_table{};
    try {
        for (std::string line; std::getline(std::cin, line);) {
            auto tokens = split(line);
            if (tokens.size() == 3) {
                if (tokens.at(1) == "+") {
                    hnd_table.create_session(tokens.at(0), tokens.at(2));
                    continue;
                } else if (tokens.at(1) == "=>") {
                    hnd_table.add_shared_ptr(tokens.at(0), tokens.at(2));
                    continue;
                } else if (tokens.at(1) == "->") {
                    hnd_table.add_weak_ptr(tokens.at(0), tokens.at(2));
                    continue;
                }
            } else if (tokens.size() == 2 && tokens.at(0) == "!") {
                hnd_table.erase(tokens.at(1));
                continue;
            } else if (tokens.size() == 1 && tokens.at(0) == "?") {
                hnd_table.display();
                continue;
            }
            std::cout << "???" << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "Unexpected exception: " << e.what() << std::endl;
        return EXIT_FAILURE;
    }
    return EXIT_SUCCESS;
}

// Reports usage instructions and return failure exit code.
int usage(char* argv0)
{
    std::cerr << "usage: " << argv0 << R"(

Maintains a graph inspired by relation of objects in an application based
on Boost.Asio. There are session and handler nodes. Each handler keeps a shared
or weak pointer to a session. There is a table of all waiting handlers, but
sessions are referenced only from handlers. So, when all handlers pointing to a
session are deleted, the session is deleted as well.

The program reads commands from stdin (H and S are string names of a handler
and a session, respectively), each command on a separate line:

H + S ... creates a new session S and a new handler H, and stores a shared
          pointer to S in H

H1 => H2 ... creates a new handler H1 with a shared pointer pointing to the
             same session as H2

H1 -> H2 ... creates a new handler H1 with a weak pointer pointing to the
             same session as H2

! H ... "executes" and deletes handler H

? ... displays list of handlers and their pointers to sessions
)";
    return EXIT_FAILURE;
}

} // namespace

// The program entry point.
int main(int argc, char* argv[])
{
    if (argc != 1)
        return usage(argv[0]);
    return run();
}
