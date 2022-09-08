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
  def search(numbers, key) do
    size = tuple_size(numbers)
    search(0, size, size, numbers, key)
  end

  defp search(start_index, end_index, max_index, _numbers, _key)
       when end_index == start_index or max_index == start_index + 1,
       do: :not_found

  defp search(start_index, end_index, max_index, numbers, target) do
    middle_index = div(start_index + end_index, 2)
    middle = elem(numbers, middle_index)

    IO.inspect(%{
      start: start_index,
      end: end_index,
      max: max_index,
      middle: middle,
      middle_index: middle_index
    })

    cond do
      target == middle -> {:ok, middle_index}
      target > middle -> search(middle_index, end_index, max_index, numbers, target)
      target < middle -> search(start_index, middle_index, max_index, numbers, target)
    end
  end
end
