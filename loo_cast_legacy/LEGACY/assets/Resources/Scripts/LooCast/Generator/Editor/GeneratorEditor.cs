using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEditor;

namespace LooCast.Generator.Editor
{
    [CustomEditor(typeof(Generator), true)]
    public class GeneratorEditor : UnityEditor.Editor
    {
        public override void OnInspectorGUI()
        {
            base.OnInspectorGUI();

            if (GUILayout.Button("Initialize"))
            {
                ((Generator)target).Initialize();
            }
        }
    } 
}
