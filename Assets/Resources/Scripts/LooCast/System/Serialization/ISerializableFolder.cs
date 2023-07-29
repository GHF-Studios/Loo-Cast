using System;
using System.IO;

namespace LooCast.System.Serialization
{
    public interface ISerializableFolder
    {
        #region Methods
        public void Serialize(out DirectoryInfo serializedFolder);
        public void Deserialize(DirectoryInfo serializedFolder);
        #endregion
    }
}
