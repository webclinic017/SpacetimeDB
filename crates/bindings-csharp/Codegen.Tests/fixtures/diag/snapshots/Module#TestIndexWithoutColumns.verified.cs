﻿//HintName: TestIndexWithoutColumns.cs
// <auto-generated />
#nullable enable

partial struct TestIndexWithoutColumns : SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader) { }

    public void WriteFields(System.IO.BinaryWriter writer) { }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<TestIndexWithoutColumns>
    {
        public TestIndexWithoutColumns Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<TestIndexWithoutColumns>(reader);

        public void Write(System.IO.BinaryWriter writer, TestIndexWithoutColumns value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType.Ref GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<TestIndexWithoutColumns>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                    new SpacetimeDB.BSATN.AggregateElement[] { }
                )
            );

        SpacetimeDB.BSATN.AlgebraicType SpacetimeDB.BSATN.IReadWrite<TestIndexWithoutColumns>.GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) => GetAlgebraicType(registrar);
    }
} // TestIndexWithoutColumns
