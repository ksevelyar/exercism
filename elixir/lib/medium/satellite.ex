defmodule Satellite do
  @typedoc """
  A tree, which can be empty, or made from a left branch, a node and a right branch
  """
  @type tree :: {} | {tree, any, tree}

  @doc """
  Build a tree from the elements given in a pre-order and in-order style
  """
  @spec build_tree(preorder :: [any], inorder :: [any]) :: {:ok, tree} | {:error, String.t()}

  def build_tree(preorder, inorder) do
    cond do
      length(preorder) != length(inorder) ->
        {:error, "traversals must have the same length"}

      Enum.sort(preorder) != Enum.sort(inorder) ->
        {:error, "traversals must have the same elements"}

      Enum.uniq(preorder) != preorder || Enum.uniq(inorder) != inorder ->
        {:error, "traversals must contain unique items"}

      true ->
        {:ok, build_node(preorder, inorder)}
    end
  end

  defp build_node([], []), do: {}
  defp build_node([x], [x]), do: {{}, x, {}}

  defp build_node([root | preorder_tail], inorder) do
    [preorder_left, _, inorder_right] = Enum.chunk_by(inorder, &(&1 == root))

    {
      build_node(preorder_left, Enum.take(preorder_tail, length(preorder_left))),
      root,
      build_node(
        Enum.slice(preorder_tail, length(preorder_left), length(inorder_right)),
        inorder_right
      )
    }
  end
end
