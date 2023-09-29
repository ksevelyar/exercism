defmodule Frequency do
  @doc """
  Count letter frequency in parallel.

  Returns a map of characters to frequencies.

  The number of worker processes to use can be set with 'workers'.
  """
  @spec frequency([String.t()], pos_integer) :: map
  def frequency(texts, workers) do
    texts
    |> Task.async_stream(fn text -> text |> remove_non_unicode_chars() |> frequencies() end,
      max_concurrency: workers
    )
    |> Enum.to_list()
    |> merge_frequencies(%{})
  end

  defp remove_non_unicode_chars(text) do
    text
    |> String.downcase()
    |> String.replace(~r/[^\p{L}]/u, "")
  end

  defp frequencies(text) do
    text |> String.graphemes() |> Enum.frequencies()
  end

  defp merge_frequencies([], acc), do: acc
  defp merge_frequencies([{:ok, frequency} | frequencies], acc) do
    acc = Map.merge(acc, frequency, fn _, frq1, frq2 -> frq1 + frq2 end)

    merge_frequencies(frequencies, acc)
  end
end
