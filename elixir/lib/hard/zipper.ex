defmodule Zipper do
  defstruct [:node, context: []]

  @doc """
  Get a zipper focused on the root node.
  """
  def from_tree(bin_tree) do
    %Zipper{node: bin_tree}
  end

  @doc """
  Get the complete tree from a zipper.
  """
  def to_tree(%{node: node, context: []}), do: node

  def to_tree(zipper) do
    zipper |> up() |> to_tree()
  end

  @doc """
  Get the value of the focus node.
  """
  def value(%{node: %{value: value}}), do: value

  @doc """
  Get the left child of the focus node, if any.
  """
  def left(%{node: %{left: nil}}), do: nil

  def left(%{node: node, context: context}) do
    %Zipper{node: node.left, context: [{node, :left} | context]}
  end

  @doc """
  Get the right child of the focus node, if any.
  """
  def right(%{node: %{right: nil}}), do: nil

  def right(%{node: node, context: context}) do
    %Zipper{node: node.right, context: [{node, :right} | context]}
  end

  @doc """
  Get the parent of the focus node, if any.
  """
  def up(%{context: []}), do: nil

  def up(%{node: node, context: [{parent, direction} | rest]}) do
    %Zipper{node: %{parent | direction => node}, context: rest}
  end

  @doc """
  Set the value of the focus node.
  """
  def set_value(%{node: node} = zipper, value) do
    %{zipper | node: %{node | value: value}}
  end

  @doc """
  Replace the left child tree of the focus node.
  """
  def set_left(%{node: node} = zipper, left) do
    %{zipper | node: %{node | left: left}}
  end

  @doc """
  Replace the right child tree of the focus node.
  """
  def set_right(%{node: node} = zipper, right) do
    %{zipper | node: %{node | right: right}}
  end
end
