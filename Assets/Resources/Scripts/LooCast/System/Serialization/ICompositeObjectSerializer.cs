namespace LooCast.System.Serialization
{
    /// <summary>
    /// CAUTION: Composite folder serializers should ALWAYS serialize to and deserialize from an XElement!
    /// </summary>
    public interface ICompositeObjectSerializer : ISerializer
    {
        #region Methods
        object Serialize(string name, object serializableCompositeObject);
        object Deserialize(object serializedCompositeObject);
        #endregion
    }
}
