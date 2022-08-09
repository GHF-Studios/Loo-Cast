using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Attribute.Editor
{
    [CustomEditor(typeof(Attribute), true)]
    public class AttributeEditor : UnityEditor.Editor
    {
        SerializedProperty Stats;

        SerializedProperty Level;
        SerializedProperty Level_OnValueChanged;
        SerializedProperty MaxLevel;
        SerializedProperty MaxLevel_OnValueChanged;
        SerializedProperty ProposedLevelChange;
        SerializedProperty ProposedLevelChange_OnValueChanged;

        void OnEnable()
        {
            Stats = serializedObject.FindProperty("Stats");

            Level = serializedObject.FindProperty("Level");
            Level_OnValueChanged = Level.FindPropertyRelative("OnValueChanged");

            MaxLevel = serializedObject.FindProperty("MaxLevel");
            MaxLevel_OnValueChanged = Level.FindPropertyRelative("OnValueChanged");

            ProposedLevelChange = serializedObject.FindProperty("ProposedLevelChange");
            ProposedLevelChange_OnValueChanged = Level.FindPropertyRelative("OnValueChanged");
        }

        public override void OnInspectorGUI()
        {
            serializedObject.Update();

            EditorGUILayout.PropertyField(Stats);

            EditorGUILayout.PropertyField(Level);
            EditorGUILayout.PropertyField(Level_OnValueChanged);

            EditorGUILayout.PropertyField(MaxLevel);
            EditorGUILayout.PropertyField(MaxLevel_OnValueChanged);

            EditorGUILayout.PropertyField(ProposedLevelChange);
            EditorGUILayout.PropertyField(ProposedLevelChange_OnValueChanged);

            serializedObject.ApplyModifiedProperties();
        }
    }
}
