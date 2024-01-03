defmodule Knapsack do
  def maximum_value(items, weight) do
    acc = Enum.reduce(items, %{0 => 0}, fn item, acc ->
      iterate(weight - item.weight, item, acc)
    end)

    acc[weight] || 0
  end

  defp iterate(max_weight, item, acc) do
    Enum.reduce(0..max_weight, acc, fn weight, acc ->
      if best_variant?(acc[weight], acc[weight + item.weight]) do
        Map.put(acc, weight + item.weight, acc[weight] + item.value)
      else
        acc
      end
    end)
  end

  defp best_variant?(cache, weight) do
    cache != nil && (weight == nil || weight < cache)
  end
end
