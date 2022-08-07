using System.Linq;
using UnityEditor;
using UnityEngine;

namespace LooCast.Data.Editor
{
    using LooCast.Variable;

    [CustomPropertyDrawer(typeof(StringVariable))]
    public class StringVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            string value = property.FindPropertyRelative("Value").stringValue;
            value = EditorGUI.TextField(position, value);
            property.FindPropertyRelative("BaseValue").stringValue = value;

            EditorGUI.EndProperty();
        }
    } 
}
