using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Universe.Editor
{
    [CustomEditor(typeof(Universe))]
    public class UniverseEditor : UnityEditor.Editor
    {
        public override void OnInspectorGUI()
        {
            Universe universe = (Universe)target;

            DrawDefaultInspector();
        }
    }
}
