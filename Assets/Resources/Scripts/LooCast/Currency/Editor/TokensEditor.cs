using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.Currency.Editor
{
    [CustomEditor(typeof(Tokens))]
    public class TokensEditor : UnityEditor.Editor
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

            Tokens tokens = (Tokens)target;
            if (GUILayout.Button("Create default data!"))
            {
                tokens.Save(true);
            }
        }
    }
}
