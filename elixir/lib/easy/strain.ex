defmodule Strain do
  @doc """
  Given a `list` of items and a function `fun`, return the list of items where
  `fun` returns true.

  Do not use `Enum.filter`.
  """
  @spec keep(list :: list(any), fun :: (any -> boolean)) :: list(any)
  def keep(list, fun), do: do_keep(list, fun, [], true)

  @doc """
  Given a `list` of items and a function `fun`, return the list of items where
  `fun` returns false.

  Do not use `Enum.reject`.
  """
  @spec discard(list :: list(any), fun :: (any -> boolean)) :: list(any)
  def discard(list, fun), do: do_keep(list, fun, [], false)

  defp do_keep([], _fun, acc, _check), do: Enum.reverse(acc)

  defp do_keep([head | tail], fun, acc, check) do
    case fun.(head) == check do
      true -> do_keep(tail, fun, [head | acc], check)
      false -> do_keep(tail, fun, acc, check)
    end
  end
end
