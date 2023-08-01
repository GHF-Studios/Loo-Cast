using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public interface ISerializableObject
    {
        #region Methods
        public void Serialize(string objectName, out XElement serializedObject);
        public void Deserialize(XElement serializedObject);
        #endregion
    }
}
