namespace LooCast.System
{
    public abstract class Factory<IdentifiableType, MetaDataType> where IdentifiableType : ILooCastObject where MetaDataType : MetaData
    {
        public abstract IdentifiableType CreateInstance(MetaDataType metadata);
    }
}