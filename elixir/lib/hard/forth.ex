defmodule Forth do
  @opaque evaluator :: any

  @doc """
  Create a new evaluator.
  """
  @spec new() :: evaluator
  def new() do
    %{words: %{}, stack: :queue.new()}
  end

  @doc """
  Evaluate an input string, updating the evaluator state.
  """
  @spec eval(evaluator, String.t()) :: evaluator
  def eval(ev, s) do
    cond do
      String.ends_with?(s, " ;") ->
        add_definition(ev, s)

      true ->
        update_stack(ev, words(s))
    end
  end

  defp update_stack(ev, words) do
    Enum.reduce(words, ev, fn word, %{stack: stack, words: words} = acc ->
      number = number(word)

      cond do
        number != nil ->
          %{acc | stack: :queue.in(number, stack)}

        true ->
          if words[word] do
            %{acc | stack: :queue.join(acc.stack, words[word])}
          else
            %{acc | stack: mutate_stack(acc.stack, word)}
          end
      end
    end)
  end

  defp words(str) do
    str
    |> String.downcase()
    |> String.split(~r/[\p{C}\p{Z}]+/u)
  end

  defp add_definition(ev, word) do
    word = String.trim(word, " ;") |> String.trim(": ")
    words = String.split(word, ~r/[\p{C}\p{Z}]+/u, parts: 2)

    case words do
      [name, values] ->
        {name, values}

        values = eval(ev, values).stack
        map = Map.put(ev.words, name, values)

        %{ev | words: map}

      _ ->
        :error
    end
  end

  defp number(num) do
    case Integer.parse(num) do
      {num, ""} -> num
      _ -> nil
    end
  end

  defp mutate_stack(queue, "+") do
    with {{:value, val1}, queue} <- :queue.out_r(queue),
         {{:value, val2}, queue} <- :queue.out_r(queue) do
      :queue.in(val1 + val2, queue)
    else
      {:empty, _queue} ->
        raise Forth.StackUnderflow
    end
  end

  defp mutate_stack(queue, "-") do
    with {{:value, val1}, queue} <- :queue.out_r(queue),
         {{:value, val2}, queue} <- :queue.out_r(queue) do
      :queue.in(val2 - val1, queue)
    else
      {:empty, _queue} ->
        raise Forth.StackUnderflow
    end
  end

  defp mutate_stack(queue, "*") do
    with {{:value, val1}, queue} <- :queue.out_r(queue),
         {{:value, val2}, queue} <- :queue.out_r(queue) do
      :queue.in(val1 * val2, queue)
    else
      {:empty, _queue} ->
        raise Forth.StackUnderflow
    end
  end

  defp mutate_stack(queue, "/") do
    with {{:value, val1}, queue} <- :queue.out_r(queue),
         {{:value, val2}, queue} <- :queue.out_r(queue) do
      :queue.in(div(val2, val1), queue)
    else
      {:empty, _queue} ->
        raise Forth.StackUnderflow
    end
  end

  defp mutate_stack(queue, "drop") do
    with {{:value, _val1}, queue} <- :queue.out_r(queue) do
      queue
    else
      {:empty, _queue} ->
        raise Forth.StackUnderflow
    end
  end

  defp mutate_stack(queue, "swap") do
    with {{:value, val1}, queue} <- :queue.out_r(queue),
         {{:value, val2}, queue} <- :queue.out_r(queue) do
      queue = :queue.in(val1, queue)
      :queue.in(val2, queue)
    else
      {:empty, _queue} ->
        raise Forth.StackUnderflow
    end
  end

  defp mutate_stack(queue, "dup") do
    with {{:value, val1}, queue} <- :queue.out_r(queue) do
      queue = :queue.in(val1, queue)
      :queue.in(val1, queue)
    else
      {:empty, _queue} ->
        raise Forth.StackUnderflow
    end
  end

  defp mutate_stack(queue, "over") do
    with {{:value, val1}, queue} <- :queue.out_r(queue),
         {{:value, val2}, queue} <- :queue.out_r(queue) do
      queue = :queue.in(val2, queue)
      queue = :queue.in(val1, queue)
      :queue.in(val2, queue)
    else
      {:empty, _queue} ->
        raise Forth.StackUnderflow
    end
  end

  @doc """
  Return the current stack as a string with the element on top of the stack
  being the rightmost element in the string.
  """
  @spec format_stack(evaluator) :: String.t()
  def format_stack(ev) do
    ev.stack
    |> :queue.to_list()
    |> Enum.join(" ")
  end

  defmodule StackUnderflow do
    defexception []
    def message(_), do: "stack underflow"
  end

  defmodule InvalidWord do
    defexception word: nil
    def message(e), do: "invalid word: #{inspect(e.word)}"
  end

  defmodule UnknownWord do
    defexception word: nil
    def message(e), do: "unknown word: #{inspect(e.word)}"
  end

  defmodule DivisionByZero do
    defexception []
    def message(_), do: "division by zero"
  end
end
