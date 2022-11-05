#include <gtest/gtest.h>

#include "variable/variable.hpp"
#include "object/object.hpp"

TEST(Variable, Basic_test)
{
    Variable val1;
    Variable val2;

    EXPECT_EQ(val1.get_flag(), Type::Undefined);
    EXPECT_EQ(val2.get_flag(), Type::Undefined);

    val1.set_undefined();
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Undefined);
    EXPECT_EQ(val2.get_flag(), Type::Undefined);

    val1.set_null();
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Null);
    EXPECT_EQ(val2.get_flag(), Type::Null);

    val1.set_number(Number(13));
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Number);
    EXPECT_EQ(val2.get_flag(), Type::Number);
    EXPECT_EQ(val1.get_number(), Number(13));
    EXPECT_EQ(val2.get_number(), Number(13));

    val1.set_boolean(true);
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Boolean);
    EXPECT_EQ(val2.get_flag(), Type::Boolean);
    EXPECT_EQ(val1.get_boolean(), true);
    EXPECT_EQ(val2.get_boolean(), true);

    val1.set_string("foo");
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::String);
    EXPECT_EQ(val2.get_flag(), Type::String);
    EXPECT_EQ(val1.get_string(), "foo");
    EXPECT_EQ(val2.get_string(), "foo");

    val1.set_object(Object());
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Object);
    EXPECT_EQ(val2.get_flag(), Type::Object);
    EXPECT_EQ(val1.get_object(), Object());
    EXPECT_EQ(val2.get_object(), Object());

    val1.set_array(Array({new Variable(), new Variable()}));
    val2 = val1;
    EXPECT_EQ(val1.get_flag(), Type::Array);
    EXPECT_EQ(val2.get_flag(), Type::Array);
    EXPECT_EQ(val1.get_array().len(), 2);
    EXPECT_EQ(val2.get_array().len(), 2);
}

TEST(Variable, to_boolean_test)
{
    Variable val;

    EXPECT_EQ(Variable(Type::Undefined).to_boolean(), false);
    EXPECT_EQ(Variable(Type::Null).to_boolean(), false);
    EXPECT_EQ(Variable(Number(1)).to_boolean(), true);
    EXPECT_EQ(Variable(true).to_boolean(), true);
    EXPECT_EQ(Variable(false).to_boolean(), false);
    EXPECT_EQ(Variable("Hello world").to_boolean(), true);
    EXPECT_EQ(Variable("").to_boolean(), false);
    EXPECT_EQ(Variable(Object()).to_boolean(), true);
    EXPECT_EQ(Variable(Array()).to_boolean(), false);
}

TEST(Variable, to_number_test)
{
    EXPECT_EQ(Variable(Type::Undefined).to_number(), Number(NumberType::NaN));
    EXPECT_EQ(Variable(Type::Null).to_number(), Number(0));
    EXPECT_EQ(Variable(Number(13)).to_number(), Number(13));
    EXPECT_EQ(Variable(true).to_number(), Number(1));
    EXPECT_EQ(Variable(false).to_number(), Number(0));
    EXPECT_EQ(Variable("Hello world").to_number(), Number(NumberType::NaN));
    EXPECT_EQ(Variable("").to_number(), Number(NumberType::NaN));
    EXPECT_EQ(Variable(Object()).to_number(), Number(NumberType::NaN));
    EXPECT_EQ(Variable(Array()).to_number(), Number(NumberType::NaN));
}

TEST(Variable, to_string_test)
{
    EXPECT_EQ(Variable(Type::Undefined).to_string(), "undefined");
    EXPECT_EQ(Variable(Type::Null).to_string(), "null");
    EXPECT_EQ(Variable(Number(NumberType::NaN)).to_string(), "NaN");
    EXPECT_EQ(Variable(true).to_string(), "true");
    EXPECT_EQ(Variable(false).to_string(), "false");
    EXPECT_EQ(Variable("Hello world").to_string(), "Hello world");
    EXPECT_EQ(Variable("").to_string(), "");
    EXPECT_EQ(Variable(Object()).to_string(), "{}");
    EXPECT_EQ(Variable(Array()).to_string(), "[]");
}

TEST(Variable, arithmetic_test)
{
    Variable res;

    // "Hello " + "world" = "Hello world"
    res = Variable("Hello ") + Variable("world");
    EXPECT_EQ(res.get_flag(), Type::String);
    EXPECT_EQ(res.get_string(), "Hello world");

    // 2 + " world" = "2.000000 world"
    res = Variable(Number(2)) + Variable(" world");
    EXPECT_EQ(res.get_flag(), Type::String);
    EXPECT_EQ(res.get_string(), "2.000000 world");

    // "Hello " + 2 = "Hello 2.000000"
    res = Variable("Hello ") + Variable(Number(2));
    EXPECT_EQ(res.get_flag(), Type::String);
    EXPECT_EQ(res.get_string(), "Hello 2.000000");

    res = Variable(Number(1)) + Variable(Number(2));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(3));

    res = Variable(Number(1)) - Variable(Number(2));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(-1));

    res = Variable(Number(1)) * Variable(Number(2));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(2));

    res = Variable(Number(2)) / Variable(Number(1));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(2));
}

TEST(Variable, logical_not_test)
{
    EXPECT_EQ(!Variable(Type::Undefined), true);
    EXPECT_EQ(!Variable(Type::Undefined), true);
    EXPECT_EQ(!Variable(true), false);
    EXPECT_EQ(!Variable(false), true);
    EXPECT_EQ(!Variable(Number(2.5)), false);
    EXPECT_EQ(!Variable(Number(0)), true);
    EXPECT_EQ(!Variable("Hello world"), false);
    EXPECT_EQ(!Variable(""), true);
    EXPECT_EQ(!Variable(Object()), false);
    EXPECT_EQ(!Variable(Array()), true);
}

TEST(Variable, logical_and_test)
{
    Variable res;

    res = Variable(true) && Variable(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = Variable(false) && Variable(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = Variable(true) && Variable(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = Variable(false) && Variable(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = Variable(Number(11)) && Variable(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = Variable(true) && Variable(Number(11));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(11));

    res = Variable(Number(0)) && Variable(true);
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(0));

    res = Variable(true) && Variable(Number(0));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(0));

    res = Variable(false) && Variable(Number(0));
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = Variable(Number(0)) && Variable(false);
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(0));
}

TEST(Variable, logical_or_test)
{
    Variable res;

    res = Variable(true) || Variable(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = Variable(false) || Variable(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = Variable(true) || Variable(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = Variable(false) || Variable(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);

    res = Variable(Number(11)) || Variable(true);
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(11));

    res = Variable(true) || Variable(Number(11));
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = Variable(Number(0)) || Variable(true);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = Variable(true) || Variable(Number(0));
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), true);

    res = Variable(false) || Variable(Number(0));
    EXPECT_EQ(res.get_flag(), Type::Number);
    EXPECT_EQ(res.get_number(), Number(0));

    res = Variable(Number(0)) || Variable(false);
    EXPECT_EQ(res.get_flag(), Type::Boolean);
    EXPECT_EQ(res.get_boolean(), false);
}

TEST(Variable, logical_eq_test)
{
    EXPECT_TRUE(Variable(Type::Undefined) == Variable(Type::Undefined));
    EXPECT_TRUE(Variable(Type::Null) == Variable(Type::Null));
    EXPECT_TRUE(Variable(true) == Variable(true));
    EXPECT_TRUE(Variable(false) == Variable(false));
    EXPECT_TRUE(Variable(Number(13)) == Variable(Number(13)));
    EXPECT_TRUE(Variable("Hello world") == Variable("Hello world"));
    EXPECT_TRUE(Variable(Object()) != Variable(Object()));
    EXPECT_TRUE(Variable(Array()) != Variable(Array()));

    EXPECT_FALSE(Variable(Type::Undefined) != Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Type::Null) != Variable(Type::Null));
    EXPECT_FALSE(Variable(true) != Variable(true));
    EXPECT_FALSE(Variable(false) != Variable(false));
    EXPECT_FALSE(Variable(Number(13)) != Variable(Number(13)));
    EXPECT_FALSE(Variable("Hello world") != Variable("Hello world"));
    EXPECT_FALSE(Variable(Object()) == Variable(Object()));
    EXPECT_FALSE(Variable(Array()) == Variable(Array()));
}

TEST(Variable, logical_cmp_test)
{
    Variable a;
    Variable b;

    // > tests
    EXPECT_TRUE(Variable(Number(2)) > Variable(Number(0)));
    EXPECT_TRUE(Variable(Number(2)) > Variable(true));
    EXPECT_TRUE(Variable(true) > Variable(Number(0)));
    EXPECT_TRUE(Variable(true) > Variable(Number(-1)));
    EXPECT_TRUE(Variable(Number(1)) > Variable(Type::Null));
    EXPECT_TRUE(Variable(true) > Variable(Type::Null));
    EXPECT_TRUE(Variable("b") > Variable("a"));

    EXPECT_FALSE(Variable(Number(2)) > Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Type::Undefined) > Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) > Variable("Hello"));
    EXPECT_FALSE(Variable("Hello") > Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) > Variable(Object()));
    EXPECT_FALSE(Variable(Object()) > Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) > Variable(Array()));
    EXPECT_FALSE(Variable(Array()) > Variable(Number(2)));

    EXPECT_FALSE(Variable(true) > Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Type::Undefined) > Variable(true));
    EXPECT_FALSE(Variable(true) > Variable("Hello"));
    EXPECT_FALSE(Variable("Hello") > Variable(true));
    EXPECT_FALSE(Variable(true) > Variable(Object()));
    EXPECT_FALSE(Variable(Object()) > Variable(true));
    EXPECT_FALSE(Variable(true) > Variable(Array()));
    EXPECT_FALSE(Variable(Array()) > Variable(true));

    EXPECT_FALSE(Variable(false) > Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Type::Undefined) > Variable(false));
    EXPECT_FALSE(Variable(false) > Variable("Hello"));
    EXPECT_FALSE(Variable("Hello") > Variable(false));
    EXPECT_FALSE(Variable(false) > Variable(Object()));
    EXPECT_FALSE(Variable(Object()) > Variable(false));
    EXPECT_FALSE(Variable(false) > Variable(Array()));
    EXPECT_FALSE(Variable(Array()) > Variable(false));

    EXPECT_FALSE(Variable(Type::Null) > Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Type::Undefined) > Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) > Variable("Hello"));
    EXPECT_FALSE(Variable("Hello") > Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) > Variable(Object()));
    EXPECT_FALSE(Variable(Object()) > Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) > Variable(Array()));
    EXPECT_FALSE(Variable(Array()) > Variable(Type::Null));

    EXPECT_FALSE(Variable(Number(2)) > Variable(Number(2)));
    EXPECT_FALSE(Variable(true) > Variable(true));
    EXPECT_FALSE(Variable(false) > Variable(false));
    EXPECT_FALSE(Variable("Hello") > Variable("Hello"));
    EXPECT_FALSE(Variable(Type::Null) > Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Undefined) > Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Object()) > Variable(Object()));
    EXPECT_FALSE(Variable(Array()) > Variable(Array()));

    // >= tests
    EXPECT_TRUE(Variable(Number(2)) >= Variable(Number(0)));
    EXPECT_TRUE(Variable(Number(2)) >= Variable(true));
    EXPECT_TRUE(Variable(true) >= Variable(Number(0)));
    EXPECT_TRUE(Variable(true) >= Variable(Number(-1)));
    EXPECT_TRUE(Variable(Number(1)) >= Variable(Type::Null));
    EXPECT_TRUE(Variable(true) >= Variable(Type::Null));
    EXPECT_TRUE(Variable("b") >= Variable("a"));
    EXPECT_TRUE(Variable(Number(2)) >= Variable(Number(2)));
    EXPECT_TRUE(Variable(true) >= Variable(true));
    EXPECT_TRUE(Variable(false) >= Variable(false));
    EXPECT_TRUE(Variable("Hello") >= Variable("Hello"));
    EXPECT_TRUE(Variable(Type::Null) >= Variable(Type::Null));

    EXPECT_FALSE(Variable(Number(2)) >= Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Type::Undefined) >= Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) >= Variable("Hello"));
    EXPECT_FALSE(Variable("Hello") >= Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) >= Variable(Object()));
    EXPECT_FALSE(Variable(Object()) >= Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) >= Variable(Array()));
    EXPECT_FALSE(Variable(Array()) >= Variable(Number(2)));

    EXPECT_FALSE(Variable(true) >= Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Type::Undefined) >= Variable(true));
    EXPECT_FALSE(Variable(true) >= Variable("Hello"));
    EXPECT_FALSE(Variable("Hello") >= Variable(true));
    EXPECT_FALSE(Variable(true) >= Variable(Object()));
    EXPECT_FALSE(Variable(Object()) >= Variable(true));
    EXPECT_FALSE(Variable(true) >= Variable(Array()));
    EXPECT_FALSE(Variable(Array()) >= Variable(true));

    EXPECT_FALSE(Variable(false) >= Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Type::Undefined) >= Variable(false));
    EXPECT_FALSE(Variable(false) >= Variable("Hello"));
    EXPECT_FALSE(Variable("Hello") >= Variable(false));
    EXPECT_FALSE(Variable(false) >= Variable(Object()));
    EXPECT_FALSE(Variable(Object()) >= Variable(false));
    EXPECT_FALSE(Variable(false) >= Variable(Array()));
    EXPECT_FALSE(Variable(Array()) >= Variable(false));

    EXPECT_FALSE(Variable(Type::Null) >= Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Type::Undefined) >= Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) >= Variable("Hello"));
    EXPECT_FALSE(Variable("Hello") >= Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) >= Variable(Object()));
    EXPECT_FALSE(Variable(Object()) >= Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) >= Variable(Array()));
    EXPECT_FALSE(Variable(Array()) >= Variable(Type::Null));

    EXPECT_FALSE(Variable(Type::Undefined) >= Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Object()) >= Variable(Object()));
    EXPECT_FALSE(Variable(Array()) >= Variable(Array()));

    // < tests
    EXPECT_TRUE(Variable(Number(0)) < Variable(Number(2)));
    EXPECT_TRUE(Variable(true) < Variable(Number(2)));
    EXPECT_TRUE(Variable(Number(0)) < Variable(true));
    EXPECT_TRUE(Variable(Number(-1)) < Variable(true));
    EXPECT_TRUE(Variable(Type::Null) < Variable(Number(1)));
    EXPECT_TRUE(Variable(Type::Null) < Variable(true));
    EXPECT_TRUE(Variable("a") < Variable("b"));

    EXPECT_FALSE(Variable(Type::Undefined) < Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) < Variable(Type::Undefined));
    EXPECT_FALSE(Variable("Hello") < Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) < Variable("Hello"));
    EXPECT_FALSE(Variable(Object()) < Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) < Variable(Object()));
    EXPECT_FALSE(Variable(Array()) < Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) < Variable(Array()));

    EXPECT_FALSE(Variable(Type::Undefined) < Variable(true));
    EXPECT_FALSE(Variable(true) < Variable(Type::Undefined));
    EXPECT_FALSE(Variable("Hello") < Variable(true));
    EXPECT_FALSE(Variable(true) < Variable("Hello"));
    EXPECT_FALSE(Variable(Object()) < Variable(true));
    EXPECT_FALSE(Variable(true) < Variable(Object()));
    EXPECT_FALSE(Variable(Array()) < Variable(true));
    EXPECT_FALSE(Variable(true) < Variable(Array()));

    EXPECT_FALSE(Variable(Type::Undefined) < Variable(false));
    EXPECT_FALSE(Variable(false) < Variable(Type::Undefined));
    EXPECT_FALSE(Variable("Hello") < Variable(false));
    EXPECT_FALSE(Variable(false) < Variable("Hello"));
    EXPECT_FALSE(Variable(Object()) < Variable(false));
    EXPECT_FALSE(Variable(false) < Variable(Object()));
    EXPECT_FALSE(Variable(Array()) < Variable(false));
    EXPECT_FALSE(Variable(false) < Variable(Array()));

    EXPECT_FALSE(Variable(Type::Undefined) < Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) < Variable(Type::Undefined));
    EXPECT_FALSE(Variable("Hello") < Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) < Variable("Hello"));
    EXPECT_FALSE(Variable(Object()) < Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) < Variable(Object()));
    EXPECT_FALSE(Variable(Array()) < Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) < Variable(Array()));

    EXPECT_FALSE(Variable(Number(2)) < Variable(Number(2)));
    EXPECT_FALSE(Variable(true) < Variable(true));
    EXPECT_FALSE(Variable(false) < Variable(false));
    EXPECT_FALSE(Variable("Hello") < Variable("Hello"));
    EXPECT_FALSE(Variable(Type::Null) < Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Undefined) < Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Object()) < Variable(Object()));
    EXPECT_FALSE(Variable(Array()) < Variable(Array()));

    // <= tests
    EXPECT_TRUE(Variable(Number(0)) <= Variable(Number(2)));
    EXPECT_TRUE(Variable(true) <= Variable(Number(2)));
    EXPECT_TRUE(Variable(Number(0)) <= Variable(true));
    EXPECT_TRUE(Variable(Number(-1)) <= Variable(true));
    EXPECT_TRUE(Variable(Type::Null) <= Variable(Number(1)));
    EXPECT_TRUE(Variable(Type::Null) <= Variable(true));
    EXPECT_TRUE(Variable("a") <= Variable("b"));
    EXPECT_TRUE(Variable(Number(2)) <= Variable(Number(2)));
    EXPECT_TRUE(Variable(true) <= Variable(true));
    EXPECT_TRUE(Variable(false) <= Variable(false));
    EXPECT_TRUE(Variable("Hello") <= Variable("Hello"));
    EXPECT_TRUE(Variable(Type::Null) <= Variable(Type::Null));

    EXPECT_FALSE(Variable(Type::Undefined) <= Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) <= Variable(Type::Undefined));
    EXPECT_FALSE(Variable("Hello") <= Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) <= Variable("Hello"));
    EXPECT_FALSE(Variable(Object()) <= Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) <= Variable(Object()));
    EXPECT_FALSE(Variable(Array()) <= Variable(Number(2)));
    EXPECT_FALSE(Variable(Number(2)) <= Variable(Array()));

    EXPECT_FALSE(Variable(Type::Undefined) <= Variable(true));
    EXPECT_FALSE(Variable(true) <= Variable(Type::Undefined));
    EXPECT_FALSE(Variable("Hello") <= Variable(true));
    EXPECT_FALSE(Variable(true) <= Variable("Hello"));
    EXPECT_FALSE(Variable(Object()) <= Variable(true));
    EXPECT_FALSE(Variable(true) <= Variable(Object()));
    EXPECT_FALSE(Variable(Array()) <= Variable(true));
    EXPECT_FALSE(Variable(true) <= Variable(Array()));

    EXPECT_FALSE(Variable(Type::Undefined) <= Variable(false));
    EXPECT_FALSE(Variable(false) <= Variable(Type::Undefined));
    EXPECT_FALSE(Variable("Hello") <= Variable(false));
    EXPECT_FALSE(Variable(false) <= Variable("Hello"));
    EXPECT_FALSE(Variable(Object()) <= Variable(false));
    EXPECT_FALSE(Variable(false) <= Variable(Object()));
    EXPECT_FALSE(Variable(Array()) <= Variable(false));
    EXPECT_FALSE(Variable(false) <= Variable(Array()));

    EXPECT_FALSE(Variable(Type::Undefined) <= Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) <= Variable(Type::Undefined));
    EXPECT_FALSE(Variable("Hello") <= Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) <= Variable("Hello"));
    EXPECT_FALSE(Variable(Object()) <= Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) <= Variable(Object()));
    EXPECT_FALSE(Variable(Array()) <= Variable(Type::Null));
    EXPECT_FALSE(Variable(Type::Null) <= Variable(Array()));

    EXPECT_FALSE(Variable(Type::Undefined) < Variable(Type::Undefined));
    EXPECT_FALSE(Variable(Object()) < Variable(Object()));
    EXPECT_FALSE(Variable(Array()) < Variable(Array()));
}