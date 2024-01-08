defmodule Knapsack do
  def maximum_value([], _max_weight), do: 0

  def maximum_value(items, max_weight) do
    cache =
      Enum.reduce(1..length(items), %{}, fn item_index, cache ->
        iterate(item_index, items, max_weight, cache)
      end)

    cache[length(items)][max_weight] || 0
  end

  defp iterate(item_index, items, max_weight, cache) do
    item = Enum.at(items, item_index - 1)

    Enum.reduce(0..max_weight, cache, fn weight, cache ->
      value_without_item = cache[item_index - 1][weight] || 0

      value_with_item =
        if weight >= item.weight do
          remainder_value = cache[item_index - 1][weight - item.weight] || 0

          item.value + remainder_value
        else
          0
        end

      max_value = max(value_without_item, value_with_item)

      put_in(cache, [Access.key(item_index, %{}), weight], max_value)
    end)
  end
end
