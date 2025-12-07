import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

// API Base URL - configurable for different environments
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || "http://localhost:3000";

// Types matching the server API
export interface MiningTarget {
  address: string;
  pubkey?: string;
  protocol: string;
}

export interface ProfitabilityScore {
  target_name: string;
  coin: string;
  score: number;
}

export interface MinerStats {
  wallet_address: string;
  total_shares: number;
  valid_shares: number;
  invalid_shares: number;
  hashrate: number;
  workers_count: number;
  last_seen?: string;
}

export interface Worker {
  id: number;
  miner_id: number;
  worker_name: string;
  created_at: string;
  last_seen?: string;
  hashrate: number;
  total_shares: number;
}

export interface ShareSubmission {
  wallet_address: string;
  worker_name: string;
  target_name: string;
  difficulty: number;
  valid: boolean;
}

// API client functions
const apiClient = {
  async get<T>(endpoint: string): Promise<T> {
    const response = await fetch(`${API_BASE_URL}${endpoint}`);
    if (!response.ok) {
      throw new Error(`API error: ${response.status} ${response.statusText}`);
    }
    return response.json();
  },

  async post<T>(endpoint: string, data: any): Promise<T> {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    if (!response.ok) {
      throw new Error(`API error: ${response.status} ${response.statusText}`);
    }
    return response.json();
  },
};

// React Query hooks
export const useCurrentTarget = () => {
  return useQuery({
    queryKey: ["current-target"],
    queryFn: () => apiClient.get<MiningTarget>("/api/v1/target"),
    refetchInterval: 30000, // Refresh every 30 seconds
  });
};

export const useTargets = () => {
  return useQuery({
    queryKey: ["targets"],
    queryFn: () => apiClient.get<ProfitabilityScore[]>("/api/v1/targets"),
    refetchInterval: 30000, // Refresh every 30 seconds
  });
};

export const useCurrentTargetName = () => {
  return useQuery({
    queryKey: ["current-target-name"],
    queryFn: () => apiClient.get<string>("/api/v1/targets/current"),
    refetchInterval: 5000, // Refresh every 5 seconds
  });
};

export const useMinerStats = (walletAddress: string) => {
  return useQuery({
    queryKey: ["miner-stats", walletAddress],
    queryFn: () => apiClient.get<MinerStats>(`/api/v1/miners/${walletAddress}/stats`),
    enabled: !!walletAddress,
    refetchInterval: 10000, // Refresh every 10 seconds
  });
};

export const useMinerWorkers = (walletAddress: string) => {
  return useQuery({
    queryKey: ["miner-workers", walletAddress],
    queryFn: () => apiClient.get<Worker[]>(`/api/v1/miners/${walletAddress}/workers`),
    enabled: !!walletAddress,
    refetchInterval: 10000, // Refresh every 10 seconds
  });
};

export const useRecordShare = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (share: ShareSubmission) =>
      apiClient.post<void>("/api/v1/shares", share),
    onSuccess: () => {
      // Invalidate and refetch miner stats after recording a share
      queryClient.invalidateQueries({ queryKey: ["miner-stats"] });
      queryClient.invalidateQueries({ queryKey: ["miner-workers"] });
    },
  });
};
