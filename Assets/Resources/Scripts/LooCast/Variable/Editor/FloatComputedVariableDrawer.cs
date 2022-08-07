using UnityEditor;
using UnityEngine;

namespace LooCast.Variable.Editor
{
    using LooCast.Util;

    [CustomPropertyDrawer(typeof(FloatComputedVariable))]
    public class FloatComputedVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);
            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            FloatComputedVariable floatComputedVariable = (FloatComputedVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            float value = EditorGUI.FloatField(position, floatComputedVariable.BaseValue);
            floatComputedVariable.BaseValue = value;

            EditorGUI.EndProperty();
        }
    }
}
