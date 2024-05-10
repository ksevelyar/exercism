defmodule LinkedList do
  defstruct [:next, :value]

  @doc """
  Construct a new LinkedList
  """
  def new() do
    %LinkedList{}
  end

  @doc """
  Push an item onto a LinkedList
  """
  def push(list, elem) do
    %LinkedList{value: elem, next: list}
  end

  @doc """
  Counts the number of elements in a LinkedList
  """
  def count(list) do
    count(list.next, 0)
  end

  defp count(nil, count), do: count

  defp count(list, count) do
    count(list.next, count + 1)
  end

  @doc """
  Determine if a LinkedList is empty
  """
  def empty?(list) do
    is_nil(list.next)
  end

  @doc """
  Get the value of a head of the LinkedList
  """
  def peek(list) do
    case list.value do
      nil -> {:error, :empty_list}
      val -> {:ok, val}
    end
  end

  @doc """
  Get tail of a LinkedList
  """
  def tail(list) do
    case empty?(list) do
      true -> {:error, :empty_list}
      false -> {:ok, list.next}
    end
  end

  @doc """
  Remove the head from a LinkedList
  """
  def pop(list) do
    case empty?(list) do
      true -> {:error, :empty_list}
      false -> {:ok, list.value, list.next}
    end
  end

  @doc """
  Construct a LinkedList from a stdlib List
  """
  def from_list(list) do
    list
    |> Enum.reverse()
    |> Enum.reduce(new(), fn x, acc -> push(acc, x) end)
  end

  @doc """
  Construct a stdlib List LinkedList from a LinkedList
  """
  def to_list(list) do
    to_list(list, [])
  end

  defp to_list(%{next: nil}, acc) do
    Enum.reverse(acc)
  end

  defp to_list(list, acc) do
    to_list(list.next, [list.value | acc])
  end

  @doc """
  Reverse a LinkedList
  """
  def reverse(list) do
    list |> to_list() |> Enum.reverse() |> from_list()
  end
end
