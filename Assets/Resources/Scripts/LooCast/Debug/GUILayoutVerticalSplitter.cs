using UnityEditor;
using UnityEngine;

public class GUILayoutVerticalSplitter
{
    private float splitterPosition;
    private bool isResizing = false;
    private GUIStyle splitterStyle;

    public GUILayoutVerticalSplitter(float initialPosition)
    {
        splitterPosition = initialPosition;
        splitterStyle = new GUIStyle();
        splitterStyle.normal.background = EditorGUIUtility.whiteTexture;
    }

    public void Splitter(float height)
    {
        GUILayout.Box("", splitterStyle, GUILayout.Height(height), GUILayout.ExpandWidth(true));

        var splitterRect = GUILayoutUtility.GetLastRect();
        EditorGUIUtility.AddCursorRect(splitterRect, MouseCursor.ResizeVertical);

        if (Event.current.type == EventType.MouseDown && splitterRect.Contains(Event.current.mousePosition))
        {
            isResizing = true;
        }

        if (isResizing)
        {
            splitterPosition += Event.current.delta.y;
            splitterPosition = Mathf.Clamp(splitterPosition, height, Screen.height - height); // Add this line
        }

        if (Event.current.type == EventType.MouseUp)
        {
            isResizing = false;
        }
    }

    public float GetSplitterPosition()
    {
        return splitterPosition;
    }
}