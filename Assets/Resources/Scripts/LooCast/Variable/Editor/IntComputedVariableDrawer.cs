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

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            IntComputedVariable intComputedVariable = (IntComputedVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            int value = EditorGUI.IntField(position, intComputedVariable.BaseValue);
            intComputedVariable.BaseValue = value;

            EditorGUI.EndProperty();
        }
    }
}
