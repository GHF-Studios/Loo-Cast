using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Attribute.Stat.Editor
{
    [CustomEditor(typeof(ReflexesStat))]
    public class ReflexesStatEditor : UnityEditor.Editor
    {
        SerializedProperty Level;
        SerializedProperty Level_OnValueChanged;
        SerializedProperty MaxLevel;
        SerializedProperty MaxLevel_OnValueChanged;
        SerializedProperty ProposedLevelChange;
        SerializedProperty ProposedLevelChange_OnValueChanged;

        SerializedProperty ConsecutiveProjectileDelayMultiplier;

        void OnEnable()
        {
            Level = serializedObject.FindProperty("Level");
            Level_OnValueChanged = Level.FindPropertyRelative("OnValueChanged");

            MaxLevel = serializedObject.FindProperty("MaxLevel");
            MaxLevel_OnValueChanged = MaxLevel.FindPropertyRelative("OnValueChanged");

            ProposedLevelChange = serializedObject.FindProperty("ProposedLevelChange");
            ProposedLevelChange_OnValueChanged = ProposedLevelChange.FindPropertyRelative("OnValueChanged");

            ConsecutiveProjectileDelayMultiplier = serializedObject.FindProperty("ConsecutiveProjectileDelayMultiplier");
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

            EditorGUILayout.PropertyField(ConsecutiveProjectileDelayMultiplier);

            serializedObject.ApplyModifiedProperties();
        }
    }
}
