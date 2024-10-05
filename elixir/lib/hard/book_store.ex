defmodule BookStore do
  @typedoc "A book is represented by its number in the 5-book series"
  @type book :: 1 | 2 | 3 | 4 | 5

  @regular_price 800
  @batch_discounts %{
    1 => @regular_price,
    2 => trunc(@regular_price * 0.95),
    3 => trunc(@regular_price * 0.90),
    4 => trunc(@regular_price * 0.80),
    5 => trunc(@regular_price * 0.75)
  }
  @batch_limit 5

  @doc """
  Calculate lowest price (in cents) for a shopping basket containing books.
  """
  @spec total(basket :: [book]) :: integer
  def total(basket) do
    2..@batch_limit
    |> Enum.map(fn limit -> basket |> sort_by_frequency() |> possible_sums(limit) end)
    |> Enum.min()
  end

  defp possible_sets(basket, set_picker_fn, limit) do
    Enum.reduce(basket, [], fn book, sets ->
      available_sets = Enum.filter(sets, fn set -> book not in set and length(set) < limit end)

      case available_sets do
        [] ->
          [[book] | sets]

        _ ->
          available_set = set_picker_fn.(available_sets)

          Enum.reduce_while(sets, [], fn set, acc ->
            if set == available_set do
              {:halt, Enum.reverse([[book | set] | acc] ++ Enum.drop(sets, length(acc) + 1))}
            else
              {:cont, [set | acc]}
            end
          end)
      end
    end)
  end

  defp possible_sums(basket, limit) do
    possible_sets_min = basket |> possible_sets(&Enum.min/1, limit) |> calc_total()
    possible_sets_max = basket |> possible_sets(&Enum.max/1, limit) |> calc_total()

    min(possible_sets_min, possible_sets_max)
  end

  defp calc_total(sets) do
    sets
    |> Enum.map(fn set ->
      length = length(set)
      @batch_discounts[length] * length
    end)
    |> Enum.sum()
  end

  defp sort_by_frequency(list) do
    frequencies = Enum.frequencies(list)

    list
    |> Enum.uniq()
    |> Enum.sort_by(&frequencies[&1], :desc)
    |> Enum.flat_map(&List.duplicate(&1, frequencies[&1]))
  end
end
