using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Attribute.Stat.Editor
{
    [CustomEditor(typeof(EnduranceStat))]
    public class EnduranceStatEditor : UnityEditor.Editor
    {
        SerializedProperty Level;
        SerializedProperty Level_OnValueChanged;
        SerializedProperty MaxLevel;
        SerializedProperty MaxLevel_OnValueChanged;
        SerializedProperty ProposedLevelChange;
        SerializedProperty ProposedLevelChange_OnValueChanged;

        SerializedProperty EnergyRegenerationMultiplier;

        void OnEnable()
        {
            Level = serializedObject.FindProperty("Level");
            Level_OnValueChanged = Level.FindPropertyRelative("OnValueChanged");

            MaxLevel = serializedObject.FindProperty("MaxLevel");
            MaxLevel_OnValueChanged = MaxLevel.FindPropertyRelative("OnValueChanged");

            ProposedLevelChange = serializedObject.FindProperty("ProposedLevelChange");
            ProposedLevelChange_OnValueChanged = ProposedLevelChange.FindPropertyRelative("OnValueChanged");

            EnergyRegenerationMultiplier = serializedObject.FindProperty("EnergyRegenerationMultiplier");
        }

        public override void OnInspectorGUI()
        {
            serializedObject.Update();

            EditorGUILayout.PropertyField(Level);
            EditorGUILayout.PropertyField(Level_OnValueChanged);

            EditorGUILayout.PropertyField(MaxLevel);
            EditorGUILayout.PropertyField(MaxLevel_OnValueChanged);

            EditorGUILayout.PropertyField(ProposedLevelChange);
            EditorGUILayout.PropertyField(ProposedLevelChange_OnValueChanged);

            EditorGUILayout.PropertyField(EnergyRegenerationMultiplier);

            serializedObject.ApplyModifiedProperties();
        }
    }
}
