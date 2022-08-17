using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Currency.Editor
{
    [CustomEditor(typeof(Coins))]
    public class CoinsEditor : UnityEditor.Editor
    {
        SerializedProperty Balance;
        SerializedProperty ProposedBalanceChange;

        void OnEnable()
        {
            Balance = serializedObject.FindProperty("Balance");
            ProposedBalanceChange = serializedObject.FindProperty("ProposedBalanceChange");
        }

        public override void OnInspectorGUI()
        {
            serializedObject.Update();

            EditorGUILayout.PropertyField(Balance);
            EditorGUILayout.PropertyField(ProposedBalanceChange);

            serializedObject.ApplyModifiedProperties();

            Coins coins = (Coins)target;
            if (GUILayout.Button("Create default data!"))
            {
                coins.Save(true);
            }
        }
    }
}
