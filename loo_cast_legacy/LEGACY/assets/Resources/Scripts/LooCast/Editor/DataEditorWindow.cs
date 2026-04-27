using System;
using UnityEngine;
using UnityEditor;
using UnityEditor.PackageManager.UI;

namespace LooCast.Editor
{
    using Data;
    
    public class DataEditorWindow : EditorWindow
    {
        [MenuItem("Window/Loo Cast/Data", priority = 1)]
        public static void ShowWindow()
        {
            EditorWindow.GetWindow(typeof(DataEditorWindow));
        }

        void OnGUI()
        {
            GUILayout.Label("Data Editor", EditorStyles.boldLabel);
            if (GUILayout.Button("Reset all Data!"))
            {
                Data.ResetAll();
            }
        }
    }
}
