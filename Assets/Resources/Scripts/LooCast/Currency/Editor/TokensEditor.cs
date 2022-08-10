using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Currency.Editor
{
    [CustomEditor(typeof(Tokens))]
    public class TokensEditor : UnityEditor.Editor
    {
        SerializedProperty Balance;
        SerializedProperty Balance_OnValueChanged;
        SerializedProperty ProposedBalanceChange;
        SerializedProperty ProposedBalanceChange_OnValueChanged;

        SerializedProperty MovementSpeedMultiplier;

        void OnEnable()
        {
            Balance = serializedObject.FindProperty("Balance");
            Balance_OnValueChanged = Balance.FindPropertyRelative("OnValueChanged");

            ProposedBalanceChange = serializedObject.FindProperty("ProposedBalanceChange");
            ProposedBalanceChange_OnValueChanged = ProposedBalanceChange.FindPropertyRelative("OnValueChanged");

            MovementSpeedMultiplier = serializedObject.FindProperty("MovementSpeedMultiplier");
        }

        public override void OnInspectorGUI()
        {
            serializedObject.Update();

            EditorGUILayout.PropertyField(Balance);
            EditorGUILayout.PropertyField(Balance_OnValueChanged);

            EditorGUILayout.PropertyField(ProposedBalanceChange);
            EditorGUILayout.PropertyField(ProposedBalanceChange_OnValueChanged);

            serializedObject.ApplyModifiedProperties();
        }
    }
}
