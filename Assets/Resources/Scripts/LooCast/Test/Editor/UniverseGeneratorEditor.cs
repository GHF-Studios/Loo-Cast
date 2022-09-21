using UnityEngine;
using UnityEditor;

namespace LooCast.Test.Editor
{
    [CustomEditor(typeof(UniverseGenerator))]
    public class UniverseGeneratorEditor : UnityEditor.Editor
    {
        public override void OnInspectorGUI()
        {
            UniverseGenerator universeGenerator = (UniverseGenerator)target;

            DrawDefaultInspector();

            if (GUILayout.Button("Generate"))
            {
                universeGenerator.Generate();
            }

            if (GUILayout.Button("Save"))
            {
                universeGenerator.Save();
            }

            if (GUILayout.Button("Load"))
            {
                universeGenerator.Load();
            }

            if (GUILayout.Button("Unload"))
            {
                universeGenerator.Unload();
            }
        }
    }
}