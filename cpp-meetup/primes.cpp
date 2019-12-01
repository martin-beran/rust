#include <iostream>
#include <algorithm>
#include <vector>

void primes1(std::vector<int>& cache, size_t n) {
    for (uint64_t p = 2; p < n; p++) {
        const auto boundary = std::find_if(cache.begin(), cache.end(),
                                           [&](auto c) { return c * c > p; });
        const auto divisor = std::find_if(cache.begin(), boundary,
                                          [&](auto c) { return p % c == 0;});
        if (divisor == boundary) {
            cache.push_back(p);
        }
    }
}

void primes2(std::vector<int>& cache, size_t n) {
    uint64_t boundary = 0;
    for (uint64_t p = 2; p < n; p++) {
        boundary = std::find_if(cache.begin() + boundary, cache.end(),
                                [&](auto c) { return c * c > p; }) - cache.begin();
        const auto divisor = std::find_if(cache.begin(),
                                          cache.begin() + boundary,
                                          [&](auto c) { return p % c == 0;}) - cache.begin();
        if (divisor == boundary) {
            cache.push_back(p);
        }
    }
}

void primes3(std::vector<int>& p, size_t n) {
    p.push_back(2);
    for (int i = 3; i < n; ++i) {
        bool is_p = true;
        for (auto j = p.begin(); j != p.end() && *j * *j <= i; ++j) {
            if (i % *j == 0) {
                    is_p = false;
                    break;
            }
        }
        if (is_p)
            p.push_back(i);
    }
}

void primes4(std::vector<int>& p, size_t n) {
    p.push_back(2);
    for (int i = 3; i < n; ++i) {
        bool is_p = true;
        for (auto j = p.begin(); *j * *j <= i; ++j) {
            if (i % *j == 0) {
                    is_p = false;
                    break;
            }
        }
        if (is_p)
            p.push_back(i);
    }
}

void primes5(std::vector<int>& p, size_t n) {
    p.push_back(2);
    for (int i = 3; i < n; ++i) {
        bool is_p = true;
        std::find_if(p.begin(), p.end(), [&](auto j) {
            if (j * j > i)
                return true;
            if (i % j == 0) {
                is_p = false;
                return true;
            }
            return false;
        });
        if (is_p)
            p.push_back(i);
    }
}

int main(int argc, char* argv[])
{
    const int n = 100'000'000;
    const int alg = argc > 1 ? std::stoi(argv[1]) : 4;
    std::vector<int> p;
    switch (alg) {
    case 1:
        primes1(p, n);
        break;
    case 2:
        primes2(p, n);
        break;
    case 3:
        primes3(p, n);
        break;
    case 4:
        primes4(p, n);
        break;
    case 5:
        primes5(p, n);
        break;
    }
    //for (auto i: p)
    //    std::cout << i << std::endl;
    std::cout << "Found " << p.size() << " primes" << std::endl;
    return 0;
}
