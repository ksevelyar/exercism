defmodule BoutiqueInventory do
  def sort_by_price(inventory) do
    Enum.sort_by(inventory, & &1.price)
  end

  def with_missing_price(inventory) do
    Enum.filter(inventory, &(!&1.price))
  end

  def update_names(inventory, old_word, new_word) do
    Enum.map(inventory, fn item ->
      new_name = String.replace(item.name, old_word, new_word)
      %{item | name: new_name}
    end)
  end

  def increase_quantity(item, count) do
    new_sizes =
      Enum.reduce(item.quantity_by_size, %{}, fn {size, prev_count}, sizes ->
        Map.put(sizes, size, prev_count + count)
      end)

    Map.put(item, :quantity_by_size, new_sizes)
  end

  def total_quantity(%{quantity_by_size: sizes}) do
    Enum.reduce(sizes, 0, fn {_size, quantity}, total_quantity ->
      quantity + total_quantity
    end)
  end
end
