defmodule BookStore do
  @typedoc "A book is represented by its number in the 5-book series"
  @type book :: 1 | 2 | 3 | 4 | 5

  @regular_price 800
  @batch_price %{
    1 => @regular_price,
    2 => trunc(@regular_price * 0.95),
    3 => trunc(@regular_price * 0.90),
    4 => trunc(@regular_price * 0.80),
    5 => trunc(@regular_price * 0.85)
  }
  @batch_limit 5

  @doc """
  Calculate lowest price (in cents) for a shopping basket containing books.
  """
  @spec total(basket :: [book]) :: integer
  def total(basket) do
    2..@batch_limit
    |> Enum.map(fn limit -> possible_sums(basket, limit) end)
    |> Enum.min()
  end

  defp possible_sums(basket, limit) do
    possible_sets =
      Enum.reduce(basket, [], fn book, sets ->
        available_set = Enum.find(sets, fn set -> book not in set and length(set) < limit end)

        case available_set do
          nil ->
            [[book] | sets]

          _ ->
            Enum.map(sets, fn set ->
              case set do
                ^available_set -> [book | available_set]
                _ -> set
              end
            end)
        end
      end)

    Enum.map(possible_sets, fn set ->
      length = length(set)
      @batch_price[length] * length
    end)
    |> Enum.sum()
  end
end
