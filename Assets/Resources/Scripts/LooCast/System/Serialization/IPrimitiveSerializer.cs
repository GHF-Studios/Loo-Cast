namespace LooCast.System.Serialization
{
    /// <summary>
    /// CAUTION: Primitive Serializers should ALWAYS serialize to and deserialize from an XElement!
    /// </summary>
    public interface IPrimitiveSerializer : ISerializer
    {
        #region Methods
        object Serialize(string name, object serializablePrimitive);
        object Deserialize(object serializedPrimitive);
        #endregion
    }
}
