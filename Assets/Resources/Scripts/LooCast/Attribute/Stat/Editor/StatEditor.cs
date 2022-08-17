using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Attribute.Stat.Editor
{
    [CustomEditor(typeof(Stat), true), CanEditMultipleObjects]
    public class StatEditor : UnityEditor.Editor
    {
        SerializedProperty Attribute;

        SerializedProperty Level;
        SerializedProperty MaxLevel;
        SerializedProperty ProposedLevelChange;

        void OnEnable()
        {
            Attribute = serializedObject.FindProperty("Attribute");
            Level = serializedObject.FindProperty("Level");
            MaxLevel = serializedObject.FindProperty("MaxLevel");
            ProposedLevelChange = serializedObject.FindProperty("ProposedLevelChange");
        }

        public override void OnInspectorGUI()
        {
            serializedObject.Update();

            EditorGUILayout.PropertyField(Attribute);
            EditorGUILayout.PropertyField(Level);
            EditorGUILayout.PropertyField(MaxLevel);
            EditorGUILayout.PropertyField(ProposedLevelChange);

            serializedObject.ApplyModifiedProperties();

            Stat stat = (Stat)target;
            if (GUILayout.Button("Create default data!"))
            {
                stat.Save(true);
            }
        }
    }
}
