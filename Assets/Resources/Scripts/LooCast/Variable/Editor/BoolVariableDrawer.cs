using System.Linq;
using UnityEditor;
using UnityEngine;

namespace LooCast.Data.Editor
{
    using LooCast.Variable;

    [CustomPropertyDrawer(typeof(BoolVariable))]
    public class BoolVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            bool value = property.FindPropertyRelative("Value").boolValue;
            value = EditorGUI.Toggle(position, value);
            property.FindPropertyRelative("BaseValue").boolValue = value;

            EditorGUI.EndProperty();
        }
    } 
}
