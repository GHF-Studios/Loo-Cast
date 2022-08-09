using UnityEditor;
using UnityEngine;

namespace LooCast.Variable.Editor
{
    using LooCast.Util;

    [CustomPropertyDrawer(typeof(IntComputedVariable))]
    public class IntComputedVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position.yMax -= 24.0f;
            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);
            IntComputedVariable intVariable = (IntComputedVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            intVariable.BaseValue = EditorGUI.IntField(position, intVariable.BaseValue);

            if (intVariable.IsInitialized)
            {
                position.position += Vector2.up * 18.0f;
                EditorGUI.LabelField(position, "Evaluation:\t" + intVariable.Value.ToString());
            }
            else
            {
                position.position += Vector2.up * 18.0f;
                EditorGUI.LabelField(position, "Evaluation:\tN/A");
            }

            

            EditorGUI.EndProperty();
        }

        public override float GetPropertyHeight(SerializedProperty property, GUIContent label)
        {
            return 44.0f;
        }
    }
}
