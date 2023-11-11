defmodule Series do
  @doc """
  Finds the largest product of a given number of consecutive numbers in a given string of numbers.
  """
  @spec largest_product(String.t(), non_neg_integer) :: non_neg_integer
  def largest_product(number_string, size) do
    validate_input(number_string, size)

    number_string |> digits() |> largest_product_in_chunks(size)
  end

  defp validate_input(number_string, size) do
    String.length(number_string) < size && raise ArgumentError
    size < 0 && raise ArgumentError
  end

  defp digits(number_string) do
    number_string
    |> String.to_integer()
    |> Integer.digits()
  end

  defp largest_product_in_chunks([0], _size), do: 0
  defp largest_product_in_chunks(digits, size) do
    digits
    |> Stream.chunk_every(size, 1, :discard)
    |> Enum.map(fn list -> Enum.reduce(list, &*/2) end)
    |> Enum.max()
  end
end
