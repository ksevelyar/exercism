defmodule BinarySearch do
  @doc """
    Searches for a key in the tuple using the binary search algorithm.
    It returns :not_found if the key is not in the tuple.
    Otherwise returns {:ok, index}.

    ## Examples

      iex> BinarySearch.search({}, 2)
      :not_found

      iex> BinarySearch.search({1, 3, 5}, 2)
      :not_found

      iex> BinarySearch.search({1, 3, 5}, 5)
      {:ok, 2}
  """
  @spec search(tuple, integer) :: {:ok, integer} | :not_found
  def search(numbers, target) do
    max_index = tuple_size(numbers) - 1
    search(0, max_index, numbers, target)
  end

  defp search(index, max_index, _numbers, _target) when index > max_index, do: :not_found

  defp search(index, max_index, numbers, target) do
    middle_index = div(index + max_index, 2)
    middle = elem(numbers, middle_index)

    cond do
      target == middle -> {:ok, middle_index}
      target > middle -> search(middle_index + 1, max_index, numbers, target)
      target < middle -> search(index, middle_index - 1, numbers, target)
    end
  end
end
