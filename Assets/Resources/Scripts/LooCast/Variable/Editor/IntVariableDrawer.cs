using System.Linq;
using UnityEditor;
using UnityEngine;

namespace LooCast.Data.Editor
{
    using LooCast.Variable;

    [CustomPropertyDrawer(typeof(IntVariable))]
    public class IntVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            int value = property.FindPropertyRelative("Value").intValue;
            value = EditorGUI.IntField(position, value);
            property.FindPropertyRelative("BaseValue").intValue = value;

            EditorGUI.EndProperty();
        }
    } 
}
