using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Currency.Editor
{
    [CustomEditor(typeof(Coins))]
    public class CoinsEditor : UnityEditor.Editor
    {
        SerializedProperty Balance;
        SerializedProperty Balance_OnValueChanged;
        SerializedProperty ProposedBalanceChange;
        SerializedProperty ProposedBalanceChange_OnValueChanged;

        void OnEnable()
        {
            Balance = serializedObject.FindProperty("Balance");
            Balance_OnValueChanged = Balance.FindPropertyRelative("OnValueChanged");

            ProposedBalanceChange = serializedObject.FindProperty("ProposedBalanceChange");
            ProposedBalanceChange_OnValueChanged = ProposedBalanceChange.FindPropertyRelative("OnValueChanged");
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
