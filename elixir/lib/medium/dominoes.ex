defmodule Dominoes do
  @type domino :: {1..6, 1..6}

  @doc """
  chain?/1 takes a list of domino stones and returns boolean indicating if it's
  possible to make a full chain
  """
  @spec chain?(dominoes :: [domino]) :: boolean
  def chain?([]), do: true
  def chain?([{a, b}]), do: a == b

  def chain?(dominoes) do
    Enum.any?(dominoes, fn domino ->
      valid_chain?([domino], dominoes -- [domino])
    end)
  end

  defp valid_chain?([{first, _} | chain], []) do
    {_, last} = List.last(chain)

    first == last
  end

  defp valid_chain?([{a, _b} | _tail] = chain, dominoes) do
    Enum.any?(dominoes, fn
      {b, ^a} -> valid_chain?([{b, a} | chain], dominoes -- [{b, a}])
      {^a, b} -> valid_chain?([{b, a} | chain], dominoes -- [{a, b}])
      _ -> false
    end)
  end
end
