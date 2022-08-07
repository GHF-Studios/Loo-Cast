using System.Linq;
using UnityEditor;
using UnityEngine;

namespace LooCast.Data.Editor
{
    using LooCast.Variable;

    [CustomPropertyDrawer(typeof(FloatComputedVariable))]
    public class FloatComputedVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            float value = property.FindPropertyRelative("Value").floatValue;
            value = EditorGUI.FloatField(position, value);
            property.FindPropertyRelative("BaseValue").floatValue = value;

            EditorGUI.EndProperty();
        }
    } 
}
