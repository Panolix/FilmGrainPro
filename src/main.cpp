#include <iostream>
#include <memory>
#include "core/engine.h"
#include "ui/main_window.h"

int main(int argc, char* argv[]) {
    std::cout << "FilmGrain Pro v1.0.0 - Professional Film Emulation Tool\n";
    std::cout << "Initializing...\n";
    
    try {
        // Initialize core engine
        auto engine = std::make_unique<FilmGrainEngine>();
        if (!engine->Initialize()) {
            std::cerr << "Failed to initialize FilmGrain engine\n";
            return -1;
        }
        
        // Initialize main window
        auto main_window = std::make_unique<MainWindow>(engine.get());
        if (!main_window->Initialize()) {
            std::cerr << "Failed to initialize main window\n";
            return -1;
        }
        
        std::cout << "FilmGrain Pro initialized successfully\n";
        
        // Main application loop
        while (main_window->ShouldClose() == false) {
            main_window->Update();
            main_window->Render();
        }
        
        // Cleanup
        main_window->Shutdown();
        engine->Shutdown();
        
        std::cout << "FilmGrain Pro shutdown complete\n";
        
    } catch (const std::exception& e) {
        std::cerr << "Fatal error: " << e.what() << std::endl;
        return -1;
    }
    
    return 0;
}