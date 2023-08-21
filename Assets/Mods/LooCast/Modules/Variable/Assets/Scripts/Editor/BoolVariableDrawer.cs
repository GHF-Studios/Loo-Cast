using UnityEditor;
using UnityEngine;

namespace LooCast.Variable.Editor
{
    using LooCast.Util.Editor;

    [CustomPropertyDrawer(typeof(BoolVariable))]
    public class BoolVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);
            BoolVariable boolVariable = (BoolVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            boolVariable.Value = EditorGUI.Toggle(position, boolVariable.Value);

            EditorGUI.EndProperty();
        }
    } 
}
