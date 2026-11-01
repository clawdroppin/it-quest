// main.cpp: an interactive shell for the store, like redis-cli.
//   build: g++ -std=c++20 -O2 -o kv main.cpp     (or: cl /EHsc /std:c++20 /O2 main.cpp)
//   run:   ./kv data.log
// Type HELP for commands. Everything you SET survives a restart, because it's in data.log.
#include <fstream>
#include <iostream>

#include "kv.hpp"

int main(int argc, char** argv) {
    const std::string path = argc > 1 ? argv[1] : "data.log";
    kv::Store store;
    try {
        std::ifstream existing(path, std::ios::binary);
        std::size_t records = existing ? store.replay(existing) : 0;
        std::cout << "loaded " << store.size() << " key(s) from " << records << " record(s) in " << path << "\n";
    } catch (const std::exception& e) {
        std::cerr << "error: " << e.what() << "\n";
        return 1;
    }

    // Start each session from a compacted log (one SET per key), then append every change to it.
    kv::compactFile(store, path);
    std::ofstream log(path, std::ios::binary | std::ios::app);
    store.setLog(&log);

    std::string line;
    std::cout << "kv> " << std::flush;
    while (std::getline(std::cin, line)) {
        if (line == "QUIT" || line == "quit" || line == "EXIT" || line == "exit") break;
        if (line == "HELP" || line == "help") {
            std::cout << "SET key value | GET key | DEL key | KEYS [prefix] | COUNT | QUIT\n";
        } else if (!line.empty()) {
            try {
                std::cout << kv::execute(store, kv::parseCommand(line)) << "\n";
            } catch (const std::exception& e) {
                std::cout << "(error) " << e.what() << "\n";
            }
        }
        std::cout << "kv> " << std::flush;
    }
    return 0;
}
