using System;
using System.Reflection;
using System.Linq;
using System.Collections.Generic;

namespace LooCast.System.Serialization
{
    using LooCast.System.Paths;

    public sealed class CompositeSerializer : Serializer
    {
        #region Properties

        #endregion

        #region Fields
        private StorageType storageType;
        private BinaryPrimitiveSerializer[] binaryPrimitiveSerializers;
        private Int32PrimitiveSerializer[] xmlPrimitiveSerializers;
        private CompositeSerializer[] compositeSerializers;
        #endregion

        #region Constructors
        public CompositeSerializer(Type serializableType, StorageType storageType, BinaryPrimitiveSerializer[] binaryPrimitiveSerializers, Int32PrimitiveSerializer[] xmlPrimitiveSerializers, CompositeSerializer[] compositeSerializers) : base(serializableType)
        {
            if (storageType != StorageType.File && binaryPrimitiveSerializers.Length != 0)
            {
                throw new InvalidOperationException("Binary primitive serializers can only be used within composite serializers of storage type 'File'!");
            }
            
            this.storageType = storageType;
            this.binaryPrimitiveSerializers = binaryPrimitiveSerializers;
            this.xmlPrimitiveSerializers = xmlPrimitiveSerializers;
            this.compositeSerializers = compositeSerializers;
        }
        #endregion

        #region Methods
        public void SerializeToFolder(FolderPath pathTo, object serializableFolder)
        {
            // check if the serializableFolder object is actually a folder

            if (!SerializableType.IsAssignableFrom(serializableFolder.GetType()))
            {
                throw new ArgumentException($"The serializable folder '{nameof(serializableFolder)}' is not of the serializable type '{SerializableType}' specified by the serializer!");
            }

            // get the serialization lookup table for the serializableType from the SerializationManager

            // serialize all folders via the appropriate serializers

            // serialize all files via the appropriate serializers
        }

        public void SerializeToFile(FilePath pathTo, object serializableFile)
        {
            // check if the serializableFile object is actually a file

            if (!SerializableType.IsAssignableFrom(serializableFile.GetType()))
            {
                throw new ArgumentException($"The serializable file '{nameof(serializableFile)}' is not of the serializable type '{SerializableType}' specified by the serializer!");
            }

            // get the serialization lookup table for the serializableType from the SerializationManager

            // serialize all binary files via the appropriate serializers

            // serialize all objects via the appropriate serializers
        }

        public void SerializeToObject(ObjectPath pathTo, object serializableObject)
        {
            // check if the serializableObject object is actually an object

            if (!SerializableType.IsAssignableFrom(serializableObject.GetType()))
            {
                throw new ArgumentException($"The serializable object '{nameof(serializableObject)}' is not of the serializable type '{SerializableType}' specified by the serializer!");
            }

            // get the serialization lookup table for the serializableType from the SerializationManager

            // serialize all objects via the appropriate serializers
        }
        #endregion
    }
}
