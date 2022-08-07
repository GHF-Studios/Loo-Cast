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
            FloatComputedVariable floatVariable = (FloatComputedVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            floatVariable.BaseValue = EditorGUI.FloatField(position, floatVariable.BaseValue);

            EditorGUI.EndProperty();
        }
    }
}
