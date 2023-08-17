using UnityEngine;
using NLua;
using NLua.Exceptions;
using UnityEditor;
using LooCast;
using LooCast.System;

public class LuaConsole : MonoBehaviour
{
    private Lua lua;
    private string input = "";
    private Vector2 outputScrollPos;
    private bool scrollToBottom = false;
    private Vector2 windowSize = new Vector2(1400, 700);
    private Vector2 minWindowSize = new Vector2(700, 350);
    private bool showConsole = false;
    private GUIStyle consoleStyle;
    private GUIStyle splitterStyle;
    private float splitterHeight = 10;
    private Vector2 windowPosition = new Vector2(10, 10);
    private bool dragging = false;
    private Vector2 dragOffset;
    public float splitterPosition = 0.8f;
    private float splitterPosY;
    private bool resizingSplitter;
    private Rect resizeRect;
    private bool resizingWindow = false;
    private Vector2 resizeStartPos;

    private void Awake()
    {
        lua = new Lua();
        lua.LoadCLRPackage();
        LooCastApplication.OnLogUpdated += ScrollToBottom;

        // Create the console style
        consoleStyle = new GUIStyle();
        Texture2D background = new Texture2D(1, 1);
        background.SetPixel(0, 0, new Color(0, 0, 0, 0.9f));
        background.Apply();
        consoleStyle.normal.background = background;

        // Set the title color
        consoleStyle.normal.textColor = Color.white;

        // Create the splitter style
        splitterStyle = new GUIStyle();
        Texture2D splitterBackground = new Texture2D(1, 1);
        splitterBackground.SetPixel(0, 0, Color.white);
        splitterBackground.Apply();
        splitterStyle.normal.background = splitterBackground;

        // Initialize the splitter position
        splitterPosY = windowSize.y * splitterPosition - 15;
    }

    private void Update()
    {
        if (Input.GetKeyDown(KeyCode.F1))
        {
            showConsole = !showConsole;
            if (showConsole)
            {
                scrollToBottom = true;
            }
        }
    }

    private void OnGUI()
    {
        if (!showConsole) return;

        Rect windowRect = new Rect(windowPosition.x, windowPosition.y, windowSize.x, windowSize.y);
        GUI.Window(0, windowRect, DrawConsole, "", consoleStyle);

        // Handle window resizing
        resizeRect = new Rect(windowRect.x + windowRect.width - 20, windowRect.y + windowRect.height - 20, 20, 20);
        EditorGUIUtility.AddCursorRect(resizeRect, MouseCursor.ResizeUpLeft);
        if (Event.current.type == EventType.MouseDown && resizeRect.Contains(Event.current.mousePosition))
        {
            resizingWindow = true;
            resizeStartPos = Event.current.mousePosition;
        }

        if (resizingWindow)
        {
            float newWidth = Mathf.Clamp(Event.current.mousePosition.x - windowPosition.x, minWindowSize.x, float.MaxValue);
            float newHeight = Mathf.Clamp(Event.current.mousePosition.y - windowPosition.y, minWindowSize.y, float.MaxValue);
            splitterPosY = splitterPosY * newHeight / windowSize.y;
            windowSize.x = newWidth;
            windowSize.y = newHeight;
        }

        if (Input.GetMouseButtonUp(0))
        {
            resizingWindow = false;
        }

        // Handle window dragging
        if (dragging)
        {
            windowPosition = new Vector2(
                Mathf.Clamp(Input.mousePosition.x - dragOffset.x, 0, Screen.width - windowSize.x),
                Mathf.Clamp(Screen.height - Input.mousePosition.y - dragOffset.y, 0, Screen.height - windowSize.y)
            );

            if (Input.GetMouseButtonUp(0))
            {
                dragging = false;
            }
        }

        if (scrollToBottom)
        {
            ScrollToBottom();
            scrollToBottom = false;
        }
    }

    private void DrawConsole(int windowID)
    {
        float outputHeight = windowSize.y * splitterPosition - splitterHeight - 5;
        float contentHeight = GUI.skin.label.CalcHeight(new GUIContent(LooCastApplication.Log), windowSize.x - 40);

        // Handle window dragging
        EditorGUIUtility.AddCursorRect(new Rect(0, 0, windowSize.x, 20), MouseCursor.Pan);
        if (Event.current.type == EventType.MouseDown && Event.current.mousePosition.y < 20)
        {
            dragging = true;
            dragOffset = new Vector2(
                Input.mousePosition.x - windowPosition.x,
                Screen.height - Input.mousePosition.y - windowPosition.y
            );
        }

        // Output area
        outputScrollPos = GUI.BeginScrollView(new Rect(10, 20, windowSize.x - 20, splitterPosY - 10),
            outputScrollPos, new Rect(0, 0, windowSize.x - 40, contentHeight));
        GUI.Label(new Rect(0, 0, windowSize.x - 40, contentHeight), LooCastApplication.Log);
        GUI.EndScrollView();

        // Splitter
        EditorGUIUtility.AddCursorRect(new Rect(10, splitterPosY, windowSize.x - 20, splitterHeight), MouseCursor.ResizeVertical);
        if (Event.current.type == EventType.MouseDown && Event.current.mousePosition.y > splitterPosY && Event.current.mousePosition.y < splitterPosY + splitterHeight)
        {
            resizingSplitter = true;
        }

        if (resizingSplitter)
        {
            splitterPosY = Mathf.Clamp(Event.current.mousePosition.y, 100, windowSize.y - 100);
        }

        if (Input.GetMouseButtonUp(0))
        {
            resizingSplitter = false;
        }

        // Draw the splitter
        GUI.Box(new Rect(10, splitterPosY, windowSize.x - 40, splitterHeight), GUIContent.none, splitterStyle);

        // Input area
        input = GUI.TextField(new Rect(10, splitterPosY + splitterHeight + 5, windowSize.x - 70, windowSize.y - splitterPosY - splitterHeight - 15), input);

        // Check for Shift + Enter key press
        if (Event.current.shift && Event.current.keyCode == KeyCode.Return && Event.current.type != EventType.Layout)
        {
            if (!StringUtil.IsEmpty(input))
            {
                ExecuteLuaCode(input);
                input = "";
            }

            Event.current.Use(); // Consume the event so it doesn't trigger other actions
        }

        // Title with margin
        GUI.Label(new Rect(6, 2, windowSize.x - 20, 20), "Lua Console", consoleStyle);
    }

    private void ScrollToBottom()
    {
        float contentHeight = GUI.skin.label.CalcHeight(new GUIContent(LooCastApplication.Log), windowSize.x - 40);
        outputScrollPos.y = contentHeight;
    }

    private void ExecuteLuaCode(string code)
    {
        try
        {
            object result = lua.DoString(code)[0];
            LooCastApplication.UpdateLog($"[Lua Console] {result}");
        }
        catch (LuaException ex)
        {
            LooCastApplication.UpdateLog($"[Lua Console] Error executing Lua code: {ex.Message}");
        }
    }
}
