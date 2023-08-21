using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Attribute.Editor
{
    [CustomEditor(typeof(Attribute), true), CanEditMultipleObjects]
    public class AttributeEditor : UnityEditor.Editor
    {
        SerializedProperty Stats;
        SerializedProperty Level;
        SerializedProperty MaxLevel;
        SerializedProperty ProposedLevelChange;

        void OnEnable()
        {
            Stats = serializedObject.FindProperty("Stats");
            Level = serializedObject.FindProperty("Level");
            MaxLevel = serializedObject.FindProperty("MaxLevel");
            ProposedLevelChange = serializedObject.FindProperty("ProposedLevelChange");
        }

        public override void OnInspectorGUI()
        {
            serializedObject.Update();

            EditorGUILayout.PropertyField(Stats);
            EditorGUILayout.PropertyField(Level);
            EditorGUILayout.PropertyField(MaxLevel);
            EditorGUILayout.PropertyField(ProposedLevelChange);

            serializedObject.ApplyModifiedProperties();
        }
    }
}
