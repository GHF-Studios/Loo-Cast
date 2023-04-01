namespace LooCast.System.Factories
{
    using global::LooCast.System.MetaData;

    public class SystemObjectFactory : Factory<SystemObject, SystemObjectMetaData>
    {
        public override SystemObject CreateInstance(SystemObjectMetaData metadata)
        {
            return new SystemObject(metadata);
        }
    }
}