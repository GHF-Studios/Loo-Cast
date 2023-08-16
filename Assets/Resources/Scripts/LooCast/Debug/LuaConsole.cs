using UnityEngine;
using NLua;
using NLua.Exceptions;
using LooCast;
using LooCast.System;
using UnityEditor;

public class LuaConsole : MonoBehaviour
{
    #region Fields
    private bool showConsole = false;
    private Rect consoleWindowRect = new Rect(20, 20, 400, 300);
    private Vector2 consoleScrollPosition = Vector2.zero;
    private string consoleInput = "";
    private string consoleOutput = "";
    private const float resizeHandleSize = 15f;
    private Vector2 resizeStart = Vector2.zero;
    private bool isResizing = false;
    private GUILayoutVerticalSplitter verticalSplitter;
    private Lua lua;
    #endregion

    #region Unity Callbacks
    private void Awake()
    {
        UpdateLog(LooCastApplication.Log);
        LooCastApplication.OnLogUpdated += UpdateLog;

        lua = new Lua();
        lua.LoadCLRPackage();

        verticalSplitter = new GUILayoutVerticalSplitter(150f);
    }

    private void Update()
    {
        if (Input.GetKeyDown(KeyCode.F1))
        {
            showConsole = !showConsole;
        }
    }

    private void OnGUI()
    {
        if (showConsole)
        {
            consoleWindowRect = GUILayout.Window(0, consoleWindowRect, ConsoleWindow, "Lua Console");
        }
    }
    #endregion

    #region Methods
    private void ConsoleWindow(int windowID)
    {
        GUILayout.BeginVertical();

        float splitterHeight = 5f; // Height of the splitter
        float inputHeight = 30f; // Fixed height for the input field
        float availableHeight = consoleWindowRect.height - inputHeight - splitterHeight - 20f; // Adjust this value as needed
        float outputHeight = verticalSplitter.GetSplitterPosition();

        if (outputHeight > availableHeight)
        {
            outputHeight = availableHeight;
        }

        consoleScrollPosition = GUILayout.BeginScrollView(consoleScrollPosition, false, true, GUILayout.Height(outputHeight));
        GUILayout.TextArea(consoleOutput, GUILayout.ExpandHeight(true));
        GUILayout.EndScrollView();

        verticalSplitter.Splitter(5f);

        consoleInput = GUILayout.TextField(consoleInput, GUILayout.Height(inputHeight));

        if (GUILayout.Button("Execute") || (Event.current.isKey && Event.current.keyCode == KeyCode.Return))
        {
            if (!StringUtil.IsEmpty(consoleInput))
            {
                ExecuteLuaCode(consoleInput);
            }
        }

        GUILayout.EndVertical();
        HandleResizeEvents();
        GUI.DragWindow(new Rect(0, 0, consoleWindowRect.width, 20));
    }

    private void HandleResizeEvents()
    {
        Rect resizeHandle = new Rect(consoleWindowRect.width - resizeHandleSize, consoleWindowRect.height - resizeHandleSize, resizeHandleSize, resizeHandleSize);

        GUI.DrawTexture(resizeHandle, EditorGUIUtility.whiteTexture);
        EditorGUIUtility.AddCursorRect(resizeHandle, MouseCursor.ResizeUpLeft);

        if (Event.current.type == EventType.MouseDown && resizeHandle.Contains(Event.current.mousePosition))
        {
            isResizing = true;
            resizeStart = Event.current.mousePosition;
        }

        if (isResizing)
        {
            consoleWindowRect.width += Event.current.mousePosition.x - resizeStart.x;
            consoleWindowRect.height += Event.current.mousePosition.y - resizeStart.y;
            resizeStart = Event.current.mousePosition;
        }

        if (Event.current.type == EventType.MouseUp)
            isResizing = false;
    }

    public void ExecuteLuaCode(string code)
    {
        try
        {
            object result = lua.DoString(code)[0];
            consoleOutput += "\n" + result;
        }
        catch (LuaException ex)
        {
            consoleOutput += "\nError executing Lua code: " + ex.Message;
        }
    }

    private void UpdateLog(string log)
    {
        consoleOutput = log;
    }
    #endregion
}