using UnityEditor;
using UnityEditorInternal;
using UnityEngine;

namespace LooCast.Variable.Editor
{
    using LooCast.Util.Editor;

    [CustomPropertyDrawer(typeof(IntVariable))]
    public class IntVariableDrawer : PropertyDrawer
    {
        private UnityEventDrawer eventDrawer;

        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);
            IntVariable intVariable = (IntVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            intVariable.Value = EditorGUI.IntField(position, intVariable.Value);

            EditorGUI.EndProperty();
        }
    } 
}
