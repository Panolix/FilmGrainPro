#pragma once

#include <cstddef>
#include <string>

/**
 * Performance monitoring utilities for FilmGrain Pro
 */
class PerformanceMonitor {
public:
    // Memory monitoring
    static size_t GetMemoryUsageMB();
    static size_t GetPeakMemoryUsageMB();
    
    // CPU monitoring
    static float GetCPUUsagePercent();
    
    // GPU monitoring (placeholder for now)
    static float GetGPUUsagePercent();
    static size_t GetGPUMemoryUsageMB();
    
    // Timing utilities
    static double GetCurrentTimeSeconds();
    static void StartTimer(const std::string& name);
    static double EndTimer(const std::string& name);
    
private:
    PerformanceMonitor() = delete;
};